import { useEffect, useRef, useCallback, useState } from 'react';
import type { WsMessage, SendMessageRequest } from '@/types/websocket';
import { useAuthStore } from '@/stores/authStore';
import camelcaseKeys from "camelcase-keys";

const getWsUrl = (path: string, token?: string): string => {
    if (typeof window === 'undefined') return '';
    const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
    const tokenParam = token ? `?token=${token}` : '';
    return `${protocol}//${location.host}${path}${tokenParam}`;
};

export const useWebSocket = (
    path: string,
    onMessage: (data: WsMessage) => void,
) => {
    const ws = useRef<WebSocket | null>(null);
    const [isConnected, setIsConnected] = useState(false);
    const reconnectTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);
    const onMessageRef = useRef(onMessage);

    useEffect(() => {
        onMessageRef.current = onMessage;
    });

    const token = useAuthStore(state => state.currentAccount?.token);

    useEffect(() => {
        const url = getWsUrl(path, token);

        if (!url || !token) return;

        let cancelled = false;

        const connect = () => {
            if (cancelled) return;

            ws.current?.close(1000, 'Reconnecting');
            const socket = new WebSocket(url);
            ws.current = socket;

            socket.onopen = () => {
                if (cancelled) { socket.close(1000, 'Cancelled'); return; }
                console.log('✅ WS connected:', url);
                setIsConnected(true);
            };

            socket.onmessage = (event) => {
                try {
                    const data: WsMessage = camelcaseKeys(JSON.parse(event.data), { deep: true });
                    onMessageRef.current(data);
                } catch (e) {
                    console.error('WS parse error:', e);
                }
            };

            socket.onclose = (event) => {
                console.log('❌ WS close:', event.code, event.reason);
                setIsConnected(false);
                if (!cancelled && event.code !== 1000) {
                    reconnectTimeout.current = setTimeout(connect, 3000);
                }
            };

            socket.onerror = (e) => console.error('❌ WS error:', e);
        };

        connect();

        return () => {
            cancelled = true;
            if (reconnectTimeout.current) clearTimeout(reconnectTimeout.current);
            ws.current?.close(1000, 'Unmount');
            ws.current = null;
        };
    }, [path, token]);

    const toSnakeCase = (str: string): string =>
        str.replace(/[A-Z]/g, (c) => `_${c.toLowerCase()}`);

    const toSnakeCaseKeys = (obj: Record<string, unknown>): Record<string, unknown> =>
        Object.fromEntries(
            Object.entries(obj).map(([k, v]) => [toSnakeCase(k), v])
        );

    const sendMessage = useCallback((req: Omit<SendMessageRequest, 'type'>) => {
        if (ws.current?.readyState === WebSocket.OPEN) {
            const payload = toSnakeCaseKeys({ type: 'new_message', ...req } as Record<string, unknown>);
            ws.current.send(JSON.stringify(payload));
        } else {
            console.warn('WS not ready, state:', ws.current?.readyState);
        }
    }, []);

    return { sendMessage, isConnected };
};
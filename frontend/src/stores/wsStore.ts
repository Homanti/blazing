import { create } from 'zustand';
import camelcaseKeys from 'camelcase-keys';
import snakecaseKeys from 'snakecase-keys';
import type { WsMessage, SendMessageRequest } from '@/types/websocket';
import { API_URL } from '@/configs/api.config';

const getWsUrl = (path: string, token: string): string => {
    const protocol = API_URL.protocol === 'https:' ? 'wss:' : 'ws:';
    return `${protocol}//${API_URL.host}/api/v1${path}?token=${token}`;
};

type MessageHandler = (data: WsMessage) => void;

interface WsStore {
    sockets: Map<string, WebSocket>;
    listeners: Map<string, Set<MessageHandler>>;

    connect: (path: string, token: string) => void;
    disconnect: (path: string) => void;
    disconnectAll: () => void;
    send: (path: string, req: Omit<SendMessageRequest, 'type'>) => void;
    subscribe: (path: string, handler: MessageHandler) => () => void;
    isConnected: (path: string) => boolean;
}

export const useWsStore = create<WsStore>((set, get) => ({
    sockets: new Map(),
    listeners: new Map(),

    connect: (path, token) => {
        const { sockets } = get();

        const existing = sockets.get(path);
        if (existing && existing.readyState === WebSocket.OPEN) return;

        const ws = new WebSocket(getWsUrl(path, token));

        ws.onopen = () => {
            console.log(`✅ WS connected: ${path}`);
            set(state => ({
                sockets: new Map(state.sockets).set(path, ws)
            }));
        };

        ws.onmessage = (event) => {
            try {
                const data: WsMessage = camelcaseKeys(JSON.parse(event.data), { deep: true });
                get().listeners.get(path)?.forEach(fn => fn(data));
            } catch (e) {
                console.error('WS parse error:', e);
            }
        };

        ws.onclose = (event) => {
            console.log(`❌ WS closed: ${path}`, event.code);
            set(state => {
                const next = new Map(state.sockets);
                next.delete(path);
                return { sockets: next };
            });

            if (event.code !== 1000) {
                setTimeout(() => get().connect(path, token), 3000);
            }
        };

        ws.onerror = (e) => console.error(`❌ WS error: ${path}`, e);

        set(state => ({
            sockets: new Map(state.sockets).set(path, ws)
        }));
    },

    disconnect: (path) => {
        const ws = get().sockets.get(path);
        ws?.close(1000, 'Disconnect');
        set(state => {
            const next = new Map(state.sockets);
            next.delete(path);
            return { sockets: next };
        });
    },

    disconnectAll: () => {
        get().sockets.forEach(ws => ws.close(1000, 'Logout'));
        set({ sockets: new Map() });
    },

    send: (path, req) => {
        const ws = get().sockets.get(path);
        if (ws?.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify(
                snakecaseKeys({ type: 'new_message', ...req }, { deep: true })
            ));
        } else {
            console.warn(`WS not ready: ${path}`);
        }
    },

    subscribe: (path, handler) => {
        const { listeners } = get();
        if (!listeners.has(path)) listeners.set(path, new Set());
        listeners.get(path)!.add(handler);
        return () => listeners.get(path)?.delete(handler);
    },

    isConnected: (path) => {
        const ws = get().sockets.get(path);
        return ws?.readyState === WebSocket.OPEN;
    },
}));
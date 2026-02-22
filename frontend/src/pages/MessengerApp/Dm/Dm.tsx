import {useEffect, useState, useCallback, useRef} from "react";
import { type Message } from "@/types/message";
import { useLocation } from "react-router-dom";
import styles from "./Dm.module.scss";
import apiClient from "@/utils/api";
import { useWebSocket } from "@/hooks/useWebSocket.ts";
import type { WsMessage } from "@/types/websocket.tsx";
import Textarea from "@/components/Textarea/Textarea.tsx";
import IconButton from "@/components/IconButton/IconButton.tsx";
import {Send} from "lucide-react";
import MessageItem from "@/pages/MessengerApp/Dm/MessageItem/MessageItem.tsx";

function Dm() {
    const [messages, setMessages] = useState<Message[]>([]);
    const location = useLocation();
    const channelId = location.pathname.split("/")[3];

    const bottomRef = useRef<HTMLDivElement>(null);
    const messagesRef = useRef<HTMLDivElement>(null)

    const [input, setInput] = useState('');

    const handleMessage = useCallback((data: WsMessage) => {
        switch (data.type) {
            case 'message_created':
                setMessages((prev) => [...prev, data.message]);
                break;
        }
    }, []);

    const { sendMessage, isConnected } = useWebSocket(
        `/api/v1/chat/ws`,
        handleMessage
    );

    const sendMessageHandler = useCallback(() => {
        sendMessage({
            content: input,
            channelId: channelId
        })
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [input, sendMessage, channelId]);

    useEffect(() => {
        if (!isConnected || !channelId) return;

        apiClient.get("/api/v1/chat/history", {
            params: { channel_id: channelId },
        })
            .then(res => setMessages((res.data || []).reverse()))
            .catch(err => {
                console.error("Failed to fetch messages:", err);
                setMessages([]);
            });
    }, [channelId, isConnected]);

    useEffect(() => {
        const messagesElement = messagesRef.current;

        if (!messagesElement) return;

        const isAtBottom =
            messagesElement.scrollHeight - messagesElement.scrollTop - messagesElement.clientHeight < 100;
        if (isAtBottom) {
            bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
        }
    }, [messages]);

    return (
        <main>
            <section className={styles.chat}>
                <div className={styles.messages} ref={messagesRef}>
                    {messages.map((message) => (
                        <MessageItem key={message.id} message={message} />
                    ))}

                    <div ref={bottomRef} />
                </div>

                <form className={styles.form} onSubmit={(e) => e.preventDefault()}>
                    <Textarea className={styles.textarea} value={input} onChange={(e) => setInput(e.target.value)} />
                    <IconButton variant={"outlined"} onClick={sendMessageHandler}>
                        <Send />
                    </IconButton>
                </form>
            </section>
        </main>
    );
}

export default Dm;
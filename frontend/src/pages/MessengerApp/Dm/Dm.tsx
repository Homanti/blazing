import {useEffect, useState, useCallback, useRef} from "react";
import {type MessageWithAuthor} from "@/types/message";
import { useLocation } from "react-router-dom";
import styles from "./Dm.module.scss";
import apiClient from "@/utils/api";
import Textarea from "@/components/Textarea/Textarea.tsx";
import IconButton from "@/components/IconButton/IconButton.tsx";
import {Send} from "lucide-react";
import MessageItem from "@/pages/MessengerApp/Dm/MessageItem/MessageItem.tsx";
import {useWsStore} from "@/stores/wsStore.ts";
import {flushSync} from "react-dom";

const WS_PATH = '/chat/ws';

function Dm() {
    const [messages, setMessages] = useState<MessageWithAuthor[]>([]);
    const location = useLocation();
    const channelId = location.pathname.split("/")[3];

    const bottomRef = useRef<HTMLDivElement>(null);
    const messagesRef = useRef<HTMLDivElement>(null)

    const [input, setInput] = useState('');

    const send = useWsStore(state => state.send);

    const isConnected = useWsStore(state => state.isConnected('/chat/ws'));

    useEffect(() => {
        return useWsStore.getState().subscribe(WS_PATH, (msg) => {
            switch (msg.type) {
                case 'message_created':
                    if (msg.message.message.channelId === channelId) {
                        setMessages((prev) => {
                            const exists = prev.some(m => m.message.id === msg.message.message.id);
                            if (exists) return prev;
                            return [...prev, msg.message];
                        });
                    }
                    break;
            }
        });
    }, [channelId]);

    const sendMessageHandler = useCallback(() => {
        send(WS_PATH, {
            content: input,
            channelId: channelId
        });
        setInput('');
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [input, send, channelId]);

    useEffect(() => {
        if (!isConnected || !channelId) return;

        apiClient.get("/api/v1/chat/history", {
            params: { channelId },
        })
            .then(res => {
                flushSync(() => {
                    setMessages((res.data || []).reverse());
                });
                bottomRef.current?.scrollIntoView({ behavior: 'instant' });
            })
            .catch(err => {
                console.error("Failed to fetch messages:", err);
                setMessages([]);
            });
    }, [channelId, isConnected]);

    useEffect(() => {
        const messagesElement = messagesRef.current;

        if (!messagesElement) return;

        const isAtBottom =
            messagesElement.scrollHeight - messagesElement.scrollTop - messagesElement.clientHeight < 200;
        if (isAtBottom) {
            bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
        }
    }, [messages]);

    return (
        <main>
            <section className={styles.chat}>
                <div className={styles.messages} ref={messagesRef}>
                    {messages.map((message) => (
                        <MessageItem key={message.message.id} message={message} />
                    ))}

                    <div ref={bottomRef} />
                </div>

                <form className={styles.form} onSubmit={(e) => e.preventDefault()}>
                    <Textarea onKeyDown={(e) => {
                        if (e.key === 'Enter' && !e.shiftKey) {
                            e.preventDefault();
                            sendMessageHandler();
                        }
                    }} className={styles.textarea} value={input} onChange={(e) => setInput(e.target.value)} />
                    <IconButton variant={"outlined"} onClick={sendMessageHandler}>
                        <Send />
                    </IconButton>
                </form>
            </section>
        </main>
    );
}

export default Dm;
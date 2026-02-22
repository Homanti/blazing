import type {Message} from "@/types/message.tsx";

export interface SendMessageRequest {
    channelId: string;
    content: string;
    attachments?: string[];
}

export type WsMessage =
    | {
    type: 'message';
    channelId: string;
    content: string;
}
    | {
    type: 'message_created';
    message: Message;
}
    | {
    type: 'typing_start';
    channel_id: string;
    user_id?: string;
}
    | {
    type: 'typing_stop';
    channel_id: string;
    user_id?: string;
};
export type Message = {
    id: string;
    authorId: string;
    content: string;
    attachments?: string[];
    channelId: string;
    createdAt: string;
    updatedAt: string;
};

export type Author = {
    id: string;
    username: string;
    avatarUrl: string;
    createdAt: string;
    updatedAt: string;
}

export type MessageWithAuthor = {
    message: Message;
    author: Author;
};
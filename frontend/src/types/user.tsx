export type User = {
    id: string;
    username: string;
    email: string;
    avatarUrl: string;
    createdAt: string;
    updatedAt: string;
}

export type Account = {
    user: User;
    token: string;
}
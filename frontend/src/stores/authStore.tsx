import {create} from "zustand";
import {persist} from "zustand/middleware";
import apiClient from "@/utils/api";

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

type AuthStore = {
    accounts: Account[];
    currentAccount: Account | null;

    addAccount(account: Account): void;

    login(email: string, password: string): Promise<User>;
    register(email: string, username: string, password: string): Promise<User>;
}

export const useAuthStore = create<AuthStore>() (
    persist (
        (set, get) => ({
            accounts: [],
            currentAccount: null,

            addAccount(account: Account) {
                const existingIndex = get().accounts.findIndex(acc => acc.user.id === account.user.id);

                if (existingIndex !== -1) {
                    set(state => ({
                        accounts: state.accounts.map((acc, i) =>
                            i === existingIndex ? account : acc
                        ),
                        currentAccount: account
                    }));
                } else {
                    set(state => ({
                        accounts: [...state.accounts, account],
                        currentAccount: account
                    }));
                }
            },

            login: async (email, password) => {
                const user = await apiClient.post('/api/v1/auth/login', {
                    email,
                    password
                });

                const account: Account = {
                    user: user.data.user,
                    token: user.data.token
                };

                get().addAccount(account);

                return user.data.user;
            },

            register: async (email, username, password) => {
                const user = await apiClient.post('/api/v1/auth/register', {
                    email,
                    username,
                    password
                });

                const account: Account = {
                    user: user.data.user,
                    token: user.data.token
                };

                get().addAccount(account);

                return user.data.user;
            }
        }),

        {name: 'auth'}
    ),
)
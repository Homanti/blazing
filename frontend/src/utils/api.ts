import axios, { type AxiosInstance } from 'axios';
import camelcaseKeys from 'camelcase-keys';
import {API_URL} from "@/configs/api.config.ts";
import {useAuthStore} from "@/stores/authStore.tsx";

class ApiClient {
    private api: AxiosInstance;

    constructor() {
        this.api = axios.create({
            baseURL: API_URL,
            timeout: 10000,
        });

        this.api.interceptors.request.use((config) => {
            const token = useAuthStore.getState().currentAccount?.token;
            if (token) config.headers.Authorization = `Bearer ${token}`;
            return config;
        });

        this.api.interceptors.response.use(response => {
            response.data = camelcaseKeys(response.data, { deep: true });
            return response;
        });
    }

    get instance() {
        return this.api;
    }
}

const apiClient = new ApiClient();
export default apiClient.instance;
export type { AxiosInstance };
import axios, { type AxiosInstance } from 'axios';
import camelcaseKeys from 'camelcase-keys';
import snakecaseKeys from 'snakecase-keys';
import { useAuthStore } from "@/stores/authStore.ts";
import { API_URL } from "@/configs/api.config.ts";

class ApiClient {
    private api: AxiosInstance;

    constructor() {
        this.api = axios.create({
            baseURL: API_URL.toString(),
            timeout: 10000,
        });

        this.api.interceptors.request.use((config) => {
            const token = useAuthStore.getState().currentAccount?.token;
            if (token) config.headers.Authorization = `Bearer ${token}`;

            if (config.data && typeof config.data === 'object' && !(config.data instanceof FormData)) {
                config.data = snakecaseKeys(config.data, { deep: true });
            }

            if (config.params && typeof config.params === 'object') {
                config.params = snakecaseKeys(config.params, { deep: true });
            }

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
import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';

// https://vite.dev/config/
export default defineConfig(({ mode }) => ({
    plugins: [react()],
    resolve: {
        alias: { '@': path.resolve(__dirname, './src') }
    },
    server: {
        proxy: {
            '/api': {
                target: mode === 'production'
                    ? 'https://blazing-api.up.railway.app'
                    : 'http://localhost:3000',
                ws: true,
                changeOrigin: true,
            },
        },
    }
}));
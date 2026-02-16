import Button from "@/components/Button/Button.tsx";
import Input from "@/components/Input/Input.tsx";
import styles from "./Login.module.scss";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import { type SubmitEvent } from 'react';
import {useAuthStore} from "@/stores/authStore.tsx";
import {useNavigate} from "react-router-dom";
import PAGES from "@/configs/pages.config.ts";

function Login() {
    const { t } = useTranslation();
    const navigate = useNavigate();

    const login = useAuthStore(state => state.login);
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();

        const user = await login(email, password);

        if (!user) return;

        navigate(PAGES.APP);
        console.debug(user);
    }

    return (
        <main>
            <form className={styles.loginForm} onSubmit={handleSubmit}>
                <h2 className={styles.title}>{t('auth.common.welcomeMessage')}</h2>

                {t('auth.login.title')}
                <Input placeholder={t('auth.common.email')} type={"email"} onChange={(e) => setEmail(e.target.value)} value={email} />
                <Input placeholder={t('auth.common.password')} type={"password"} onChange={(e) => setPassword(e.target.value)} value={password} />

                <Button variant={"primary"} type={"submit"} disabled={(!email || !password)}>{t('auth.login.submit')}</Button>
                <div className={styles.actions}>
                    <Button variant={"secondary"} type={"button"} onClick={() => navigate(PAGES.REGISTER)}>{t('auth.login.register')}</Button>
                    <Button variant={"outlined"} type={"button"}>{t('auth.login.forgotPassword')}</Button>
                </div>
            </form>
        </main>
    );
}

export default Login;
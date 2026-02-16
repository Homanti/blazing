import Button from "@/components/Button/Button.tsx";
import Input from "@/components/Input/Input.tsx";
import styles from "./Register.module.scss";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import { type SubmitEvent } from 'react';
import {useAuthStore} from "@/stores/authStore.tsx";
import PAGES from "@/configs/pages.config.ts";
import {useNavigate} from "react-router-dom";

function Register() {
    const { t } = useTranslation();
    const navigate = useNavigate();

    const register = useAuthStore(state => state.register);

    const [email, setEmail] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();

        const user = await register(email, username, password);

        if (!user) return;
        console.debug(user);
    }

    return (
        <main>
            <form className={styles.registerForm} onSubmit={handleSubmit}>
                <h2 className={styles.title}>{t('auth.common.welcomeMessage')}</h2>

                {t('auth.register.title')}
                <Input placeholder={t('auth.common.email')} type={"email"} onChange={(e) => setEmail(e.target.value)} value={email} />
                <Input placeholder={t('auth.common.username')} type={"text"} onChange={(e) => setUsername(e.target.value)} value={username} />
                <Input placeholder={t('auth.common.password')} type={"password"} onChange={(e) => setPassword(e.target.value)} value={password} />

                <Button variant={"primary"} type={"submit"} disabled={(!email || !username || !password)}>{t('auth.register.submit')}</Button>
                <Button variant={"secondary"} type={"button"} onClick={() => navigate(PAGES.LOGIN)}>{t('auth.register.login')}</Button>
            </form>
        </main>
    );
}

export default Register;
import Button from "@/components/Button/Button.tsx";
import styles from "./Home.module.scss";
import {useNavigate} from "react-router-dom";
import PAGES from "@/configs/pages.config.ts";

function Home() {
    const navigate = useNavigate();

    return (
        <main>
            <section className={styles.hero}>
                <h1 className={styles.hero__title}>Blazing</h1>
                <p className={styles.hero__description}>
                    Blazing - top 1 messenger btw
                </p>
                <div className={styles.hero__actions}>
                    <Button variant={"primary"}>Download Blazing</Button>
                    <Button variant={"secondary"} onClick={() => navigate(PAGES.LOGIN)}>Use Blazing in browser</Button>
                </div>
            </section>
        </main>
    );
}

export default Home;
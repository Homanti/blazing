import {Outlet} from "react-router-dom";
import styles from "./AppLayout.module.scss";
import Dock from "@/layouts/AppLayout/components/Dock/Dock.tsx";

function AppLayout() {
    return (
        <div className={styles.appLayout}>
            <Dock />
            <Outlet />
        </div>
    );
}

export default AppLayout;
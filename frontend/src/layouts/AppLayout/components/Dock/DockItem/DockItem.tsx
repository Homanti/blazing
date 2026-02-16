import styles from './DockItem.module.scss';
import {Home} from "lucide-react";
import PAGES from "@/configs/pages.config.ts";
import {useLocation, useNavigate} from "react-router-dom";

type DockItemProps = {
    route: string;
    title: string
    iconUrl?: string;
};

function DockItem({route, title, iconUrl}: DockItemProps) {
    const navigate = useNavigate();
    const location = useLocation();

    const isActive = route === location.pathname;

    return (
        <div className={`${styles.dockItem} ${isActive ? styles.active : ""}`} onClick={() => {navigate(route)}} title={title}>
            {route == PAGES.APP ? (
                <div className={styles.dockItem__icon}>
                    <Home />
                </div>
            ) : (
                <img className={styles.dockItem__icon} src={iconUrl} alt="Dock icon" />
            )}
        </div>
    );
}

export default DockItem;
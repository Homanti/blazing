import styles from './DockItem.module.scss';
import {useLocation, useNavigate} from "react-router-dom";
import IconButton from "@/components/IconButton/IconButton.tsx";

type DockItemProps = {
    route?: string;
    title: string;
    iconUrl?: string;
    icon?: React.ReactNode;
};

function DockItem({route, title, iconUrl, icon}: DockItemProps) {
    const navigate = useNavigate();
    const location = useLocation();

    const isActive = route === location.pathname;

    return (
        <IconButton variant={isActive ? "primary" : "secondary"} className={`${styles.dockItem}`} onClick={() => {if (!route) return; navigate(route)}} title={title}>
            {iconUrl ? (
                <img className={styles.dockItem__icon} src={iconUrl} alt="Dock icon" />
            ) : (
                <div className={styles.dockItem__icon}>
                    {icon}
                </div>
            )}
        </IconButton>
    );
}

export default DockItem;
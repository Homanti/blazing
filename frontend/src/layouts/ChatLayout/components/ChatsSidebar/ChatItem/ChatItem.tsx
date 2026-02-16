import {useLocation, useNavigate} from "react-router-dom";
import PAGES from "@/configs/pages.config.ts";
import Avatar from "@/components/Avatar/Avatar.tsx";
import styles from "./ChatItem.module.scss";

type ChatItemProps = {
    username: string;
    avatarUrl?: string;
    channelId: string;
};

function ChatItem({username, avatarUrl, channelId}: ChatItemProps) {
    const navigate = useNavigate();
    const location = useLocation();
    const path = PAGES.DM + `/${channelId}`;

    const isActive = path === location.pathname;

    return (
        <div className={`${styles.chatItem} ${isActive ? styles.active : ""}`} onClick={() => navigate(path)}>
            <Avatar username={username} avatarUrl={avatarUrl} />
            {username}
        </div>
    );
}

export default ChatItem;
import {useNavigate} from "react-router-dom";
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

    return (
        <div className={styles.chatItem} onClick={() => navigate(PAGES.DM + `/${channelId}`)}>
            <Avatar username={username} avatarUrl={avatarUrl} />
            {username}
        </div>
    );
}

export default ChatItem;
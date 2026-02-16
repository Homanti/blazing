import styles from './ChatsSidebar.module.scss';
import ChatItem from "@/layouts/ChatLayout/components/ChatsSidebar/ChatItem/ChatItem.tsx";

const users = [
    {username: "bebra", channelId: "1"},
    {username: "bebra", channelId: "2"},
    {username: "bebra", channelId: "3"},
    {username: "bebra", channelId: "4"},
    {username: "bebra", channelId: "5"},
]

function ChatsSidebar() {
    return (
        <div className={styles.sidebar}>
            {users.map((user) => (
                <ChatItem username={user.username} channelId={user.channelId} />
            ))}
        </div>
    );
}

export default ChatsSidebar;
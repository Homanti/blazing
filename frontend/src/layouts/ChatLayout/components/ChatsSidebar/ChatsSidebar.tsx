import styles from './ChatsSidebar.module.scss';
import ChatItem from "@/layouts/ChatLayout/components/ChatsSidebar/ChatItem/ChatItem.tsx";
import {useAuthStore} from "@/stores/authStore.tsx";
import UserProfilePanel from "@/components/UserProfilePanel/UserProfilePanel.tsx";

const users = [
    {username: "bebra", channelId: "1"},
    {username: "bebra", channelId: "2"},
    {username: "bebra", channelId: "3"},
    {username: "bebra", channelId: "4"},
    {username: "bebra", channelId: "5"},
]

function ChatsSidebar() {
    const account = useAuthStore(s => s.currentAccount);

    if (!account) return null;

    return (
        <div className={styles.sidebar}>
            <div className={styles.chats}>
                {users.map((user) => (
                    <ChatItem username={user.username} channelId={user.channelId} />
                ))}
            </div>

            <UserProfilePanel user={account.user} />
        </div>
    );
}

export default ChatsSidebar;
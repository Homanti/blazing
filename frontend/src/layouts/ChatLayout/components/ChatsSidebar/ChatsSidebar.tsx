import styles from './ChatsSidebar.module.scss';
import ChatItem from "@/layouts/ChatLayout/components/ChatsSidebar/ChatItem/ChatItem.tsx";
import {useAuthStore} from "@/stores/authStore.ts";
import UserProfilePanel from "@/components/UserProfilePanel/UserProfilePanel.tsx";

const users = [
    {username: "6e4cec20-7943-4186-a47b-145344e970f4", channelId: "6e4cec20-7943-4186-a47b-145344e970f4"},
]

function ChatsSidebar() {
    const account = useAuthStore(s => s.currentAccount);

    if (!account) return null;

    return (
        <div className={styles.sidebar}>
            <div className={styles.chats}>
                {users.map((user) => (
                    <ChatItem key={user.channelId} username={user.username} channelId={user.channelId} />
                ))}
            </div>

            <UserProfilePanel user={account.user} />
        </div>
    );
}

export default ChatsSidebar;
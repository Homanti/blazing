import styles from './ChatsSidebar.module.scss';
import ChatItem from "@/layouts/ChatLayout/components/ChatsSidebar/ChatItem/ChatItem.tsx";
import {useAuthStore} from "@/stores/authStore.tsx";
import UserProfilePanel from "@/components/UserProfilePanel/UserProfilePanel.tsx";

const users = [
    {username: "88649c79-b0bd-488d-8217-326a7e662364", channelId: "88649c79-b0bd-488d-8217-326a7e662364"},
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
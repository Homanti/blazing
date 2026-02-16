import {Outlet} from "react-router-dom";
import styles from "./ChatLayout.module.scss";
import ChatsSidebar from "@/layouts/ChatLayout/components/ChatsSidebar/ChatsSidebar.tsx";
import UserProfilePanel from "@/components/UserProfilePanel/UserProfilePanel.tsx";
import {useAuthStore} from "@/stores/authStore.tsx";

function ChatLayout() {
    const account = useAuthStore(s => s.currentAccount);

    if (!account) return null;

    return (
        <div className={styles.chatLayout}>
            <div className={styles.left}>
                <ChatsSidebar />
                <UserProfilePanel user={account.user} />
            </div>
            <Outlet />
        </div>
    );
}

export default ChatLayout;
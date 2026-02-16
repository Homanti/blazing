import {Outlet} from "react-router-dom";
import styles from "./ChatLayout.module.scss";
import ChatsSidebar from "@/layouts/ChatLayout/components/ChatsSidebar/ChatsSidebar.tsx";

function ChatLayout() {
    return (
        <div className={styles.chatLayout}>
            <div className={styles.left}>
                <ChatsSidebar />
            </div>
            <Outlet />
        </div>
    );
}

export default ChatLayout;
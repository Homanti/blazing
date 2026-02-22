import {Outlet} from "react-router-dom";
import styles from "./ChatLayout.module.scss";
import ChatsSidebar from "@/layouts/ChatLayout/components/ChatsSidebar/ChatsSidebar.tsx";

function ChatLayout() {
    return (
        <div className={styles.chatLayout}>
            <ChatsSidebar />
            <Outlet />
        </div>
    );
}

export default ChatLayout;
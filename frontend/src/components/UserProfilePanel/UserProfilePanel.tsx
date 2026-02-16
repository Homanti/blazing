import type {User} from "@/stores/authStore.tsx";
import styles from "./UserProfilePanel.module.scss";
import Avatar from "@/components/Avatar/Avatar.tsx";
import {Settings} from "lucide-react";

type UserProfilePanelProps = {
    user: User
};

function UserProfilePanel({user}: UserProfilePanelProps) {
    return (
        <div className={styles.panel}>
            <Avatar username={user.username} avatarUrl={user.avatarUrl} />
            {user.username}

            <div className={styles.actions}>
                <button className={styles.iconButton}>
                    <Settings />
                </button>
            </div>
        </div>
    );
}

export default UserProfilePanel;
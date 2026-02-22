import styles from "./UserProfilePanel.module.scss";
import Avatar from "@/components/Avatar/Avatar.tsx";
import {Settings} from "lucide-react";
import type {User} from "@/types/user.tsx";

type UserProfilePanelProps = {
    user: User
};

function UserProfilePanel({user}: UserProfilePanelProps) {
    return (
        <div className={styles.panel}>
            <div className={styles.user}>
                <Avatar username={user.username} avatarUrl={user.avatarUrl} />
                {user.username}
            </div>

            <div className={styles.actions}>
                <button className={styles.iconButton}>
                    <Settings />
                </button>
            </div>
        </div>
    );
}

export default UserProfilePanel;
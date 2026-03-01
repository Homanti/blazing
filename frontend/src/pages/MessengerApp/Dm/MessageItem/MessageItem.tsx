import type {MessageWithAuthor} from "@/types/message.tsx";
import styles from "./MessageItem.module.scss";
import Avatar from "@/components/Avatar/Avatar.tsx";

type MessageItemProps = {
    message: MessageWithAuthor;
};

function MessageItem({message}: MessageItemProps) {
    return (
        <div className={styles.message}>
            <span className={styles.avatarWrapper}>
                <Avatar avatarUrl={message.author.avatarUrl} username={message.author.username} />
            </span>

            <span className={styles.contentWrapper}>
                <p>{message.author.username}</p>
                <p>{message.message.content}</p>
            </span>
        </div>
    );
}

export default MessageItem;
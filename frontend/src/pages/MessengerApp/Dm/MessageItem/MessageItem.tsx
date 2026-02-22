import type {Message} from "@/types/message.tsx";
import styles from "./MessageItem.module.scss";
import Avatar from "@/components/Avatar/Avatar.tsx";

type MessageItemProps = {
    message: Message;
};

function MessageItem({message}: MessageItemProps) {
    return (
        <div className={styles.message}>
            <span className={styles.avatarWrapper}>
                <Avatar username={message.authorId} />
            </span>

            <span className={styles.contentWrapper} key={message.id}>
                <p>{message.authorId}</p>
                <p>{message.content}</p>
            </span>
        </div>
    );
}

export default MessageItem;
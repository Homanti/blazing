import styles from "./Avatar.module.scss";

type AvatarProps = {
    avatarUrl?: string;
    username: string;
};

function Avatar({avatarUrl, username}: AvatarProps) {
    return (
        <div className={styles.avatar}>
            {avatarUrl ? (
                <img src={avatarUrl} alt={username} />
            ) : (
                <p>{username[0]}</p>
            )}
        </div>
    );
}

export default Avatar;
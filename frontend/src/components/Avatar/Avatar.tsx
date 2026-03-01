import styles from "./Avatar.module.scss";

type AvatarProps = {
    avatarUrl?: string;
    username: string;
};

function getInitials(username: string) {
    const parts = username.trim().split(" ").filter(Boolean);
    if (parts.length === 1) return parts[0][0]?.toUpperCase() ?? "";
    return (parts[0][0] + parts[1][0]).toUpperCase();
}

function getColorFromString(str: string) {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        hash = (hash << 5) - hash + str.charCodeAt(i);
        hash |= 0;
    }
    const hue = Math.abs(hash) % 360;
    return `hsl(${hue}, 70%, 45%)`;
}

function Avatar({ avatarUrl, username }: AvatarProps) {
    const initials = getInitials(username);
    const bg = getColorFromString(username);

    return (
        <div
            className={styles.avatar}
            style={!avatarUrl ? { backgroundColor: bg } : undefined}
        >
            {avatarUrl ? (
                <img draggable={false} src={avatarUrl} alt={username} />
            ) : (
                <span>{initials}</span>
            )}
        </div>
    );
}

export default Avatar;
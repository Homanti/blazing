import type {ButtonHTMLAttributes} from "react";
import styles from "./IconButton.module.scss";

type ButtonProps = {
    variant?: 'primary' | 'secondary' | 'outlined';
    children: React.ReactNode;
    className?: string;
} & ButtonHTMLAttributes<HTMLButtonElement>;

function IconButton({ variant = 'primary', children, className, ...props }: ButtonProps) {
    const variantClass = styles[variant];

    return (
        <button className={`${styles.iconButton} ${variantClass} ${className ? className : ""}`} {...props}>
            {children}
        </button>
    );
}

export default IconButton;
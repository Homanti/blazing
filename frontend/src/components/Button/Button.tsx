import type {ButtonHTMLAttributes} from "react";
import styles from "./Button.module.scss";

type ButtonProps = {
    variant?: 'primary' | 'secondary' | 'outlined';
    children: React.ReactNode;
} & ButtonHTMLAttributes<HTMLButtonElement>;

function Button({ variant = 'primary', children, ...props }: ButtonProps) {
    const variantClass = styles[variant];

    return (
        <button className={`${styles.button} ${variantClass}`} {...props}>
            {children}
        </button>
    );
}

export default Button;
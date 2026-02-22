import styles from "./Textarea.module.scss";
import type {TextareaHTMLAttributes} from "react";

type TextareaProps = {
    className?: string;
} & TextareaHTMLAttributes<HTMLTextAreaElement>;

function Textarea({className = "", ...props}: TextareaProps) {
    return (
        <textarea className={`${styles.textarea} ${className}`} {...props}></textarea>
    );
}

export default Textarea;
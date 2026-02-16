import styles from "./Textarea.module.scss";
import type {TextareaHTMLAttributes} from "react";

type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement>

function Textarea({...props}: TextareaProps) {
    return (
        <textarea className={`${styles.textarea} ${props.className}`} {...props}></textarea>
    );
}

export default Textarea;
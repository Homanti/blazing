import styles from "./Input.module.scss";

type InputProps = React.InputHTMLAttributes<HTMLInputElement>;

function Input({...props}: InputProps) {
    return (
        <input className={`${styles.input} ${props.className}`} {...props} />
    );
}

export default Input;
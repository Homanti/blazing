import type {ReactNode} from "react";

type RequireAuthProps = {
    children: ReactNode;
};

function RequireAuth({children}: RequireAuthProps) {
    return children;
}

export default RequireAuth;
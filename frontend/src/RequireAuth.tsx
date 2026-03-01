import {type ReactNode, useEffect} from "react";
import {useAuthStore} from "@/stores/authStore.ts";
import {useWsStore} from "@/stores/wsStore.ts";

type RequireAuthProps = {
    children: ReactNode;
};

function RequireAuth({children}: RequireAuthProps) {
    const token = useAuthStore(state => state.currentAccount?.token);

    useEffect(() => {
        if (token) useWsStore.getState().connect('/chat/ws', token);
        else useWsStore.getState().disconnect('/chat/ws');
    }, [token]);

    // auth logic in future

    return children;
}

export default RequireAuth;
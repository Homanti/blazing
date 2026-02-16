import {Outlet} from "react-router-dom";

function ChatLayout() {
    return (
        <div>
            <Outlet />
        </div>
    );
}

export default ChatLayout;
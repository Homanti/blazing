import {useLocation, useRoutes} from "react-router-dom";
import {routes} from "@/routes.tsx";

function App() {
    const location = useLocation();
    const element = useRoutes(routes, location);

    return (
        <>
            {element}
        </>
    )
}

export default App

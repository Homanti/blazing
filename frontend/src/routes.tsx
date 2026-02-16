import type {RouteObject} from "react-router-dom";
import Login from "@/pages/Login/Login.tsx";
import Register from "@/pages/Register/Register.tsx";
import Home from "@/pages/Home/Home.tsx";
import Dashboard from "@/pages/MessengerApp/Dashboard/Dashboard.tsx";
import AppLayout from "@/layouts/AppLayout/AppLayout.tsx";
import RequireAuth from "@/RequireAuth.tsx";
import Guilds from "@/pages/MessengerApp/Guilds/Guilds.tsx";
import PAGES from "@/configs/pages.config.ts";
import Dm from "@/pages/MessengerApp/Dm/Dm.tsx";
import ChatLayout from "@/layouts/ChatLayout/ChatLayout.tsx";

export const authRoutes: RouteObject[] = [
    { path: '/login', element: <Login />},
    { path: '/register', element: <Register />},
]

export const publicRoutes: RouteObject[] = [
    { path: '/', element: <Home /> }
]

export const routes = [
    ...authRoutes,
    ...publicRoutes,
    {
        path: PAGES.APP,
        element: <RequireAuth><AppLayout /></RequireAuth>,
        children: [
            {
                element: <ChatLayout />,
                children: [
                    { index: true, element: <Dashboard /> },
                    { path: "dm/:id", element: <Dm /> }
                ]
            },
            { path: "guilds/:id", element: <Guilds /> }
        ]
    }

]
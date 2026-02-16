import type {RouteObject} from "react-router-dom";
import Login from "@/pages/Login/Login.tsx";

export const publicRoutes: RouteObject[] = [
    { path: '/login', element: <Login />}
]

export const privateRoutes: RouteObject[] = []

export const routes = [...publicRoutes, ...privateRoutes]
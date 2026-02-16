import type {RouteObject} from "react-router-dom";
import Login from "@/pages/Login/Login.tsx";
import Register from "@/pages/Register/Register.tsx";
import AppHome from "@/pages/AppHome/AppHome.tsx";

export const authRoutes: RouteObject[] = [
    { path: '/login', element: <Login />},
    { path: '/register', element: <Register />},
]

export const publicRoutes: RouteObject[] = []

export const privateRoutes: RouteObject[] = [
    { path: '/app', element: <AppHome /> }
]

export const routes = [
    ...authRoutes,
    ...publicRoutes,
    ...privateRoutes,
]
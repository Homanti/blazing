import PAGES from "@/configs/pages.config.ts";
import DockItem from "@/layouts/AppLayout/components/Dock/DockItem/DockItem.tsx";
import styles from "./Dock.module.scss";
import {Home, Plus} from "lucide-react";


function Dock() {
    return (
        <div className={styles.dock}>
            <DockItem route={PAGES.APP} title={"Home"} icon={<Home />} />
            <DockItem title={"Add"} icon={<Plus />} />
        </div>
    );
}

export default Dock;
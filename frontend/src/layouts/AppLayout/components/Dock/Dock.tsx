import PAGES from "@/configs/pages.config.ts";
import DockItem from "@/layouts/AppLayout/components/Dock/DockItem/DockItem.tsx";
import styles from "./Dock.module.scss";
import {Home, Plus} from "lucide-react";

const tabs = [
    { route: PAGES.APP + "/guilds/123", title: 'gaymerskaya', iconUrl: 'https://cdn.discordapp.com/icons/772065771206475787/d4f91eeb0011ced6abca7c031e45cbc8.webp?size=160&quality=lossless' },
    { route: PAGES.APP + "/guilds/321", title: 'wacorp', iconUrl: 'https://cdn.discordapp.com/icons/825059806720426014/dab50690eed4cc2179c942bf185df13f.webp?size=160&quality=lossless' },
    { route: PAGES.APP + "/guilds/424", title: 'navi', iconUrl: 'https://cdn.discordapp.com/icons/699271929605128192/a_d7534ec891b53dc23ea7a8be594669c0.webp?size=160&quality=lossless' },
];

function Dock() {
    return (
        <div className={styles.dock}>
            <DockItem route={PAGES.APP} title={"Home"} icon={<Home />} />
            <DockItem title={"Add"} icon={<Plus />} />
            {tabs.map((tab, index) => (
                <DockItem key={index} route={tab.route} title={tab.title} iconUrl={tab.iconUrl} />
            ))}
        </div>
    );
}

export default Dock;
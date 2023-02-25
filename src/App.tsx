import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";
import UFCWrapper from "./components/UFCWrapper";

export default function Home() {
    const [cachedFights, setFights] = useState<FightCard>({
        name: "NO - DATA",
        fights: [],
    } as FightCard);

    useEffect(() => {
        invoke("fetch_data", {})
            .then((data) => {
                setFights(data as FightCard);
            })
            .catch((error) => console.log(error));
    }, []);

    return (
        <main>
            <div className="grid justify-center">
                <h1 className="text-3xl font-bold">{cachedFights.name}</h1>
            </div>
            <UFCWrapper fights={cachedFights.fights}></UFCWrapper>
        </main>
    );
}

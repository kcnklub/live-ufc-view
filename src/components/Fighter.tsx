import React from "react";

interface FighterProps {
    side: string;
    fighter: Fighter;
}

const Fighter = (props: FighterProps) => {
    const fighter = props.fighter;
    const isLeft = props.side === "left";
    const justifyContent = () => {
        return isLeft ? "flex justify-end col-start-2" : "";
    };

    return (
            <>
                <div className={`p-4 col-span-3`}>
                        <div className={`${justifyContent()} rounded-full`}>
                            <img src={`${fighter.photo_src + "&h=120&w=120&scale=crop"}`} className="rounded-full border-neutral-50 border-2"/>
                        </div>
                        <div>
                            <h3 className={`${justifyContent()} text-lg`}>{fighter.name}</h3>
                            <p className={`${justifyContent()} text-sm`}>{fighter.record}</p>
                        </div>
                </div>
            </>
           );
};

export default Fighter;

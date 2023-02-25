import React from "react";

interface FighterProps {
    side: string;
    fighter: Fighter;
}

const Fighter = (props: FighterProps) => {
    const fighter = props.fighter;
    const isLeft = props.side === "left";

    const justifyContent = () => {
        return isLeft ? "flex justify-end" : "";
    };
    return (
        <div className={`p-4`}>
            <h3 className={`${justifyContent()} text-lg`}>{fighter.name}</h3>
            <p className={`${justifyContent()} text-sm`}>{fighter.record}</p>
        </div>
    );
};

export default Fighter;

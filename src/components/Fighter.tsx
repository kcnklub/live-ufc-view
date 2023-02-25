import React from "react";

interface FighterProps {
    fighter: Fighter;
}

const Fighter = (props: FighterProps) => {
    const fighter = props.fighter;
    return (
        <div className="p-4">
            <h3 className="text-lg">{fighter.name}</h3>
            <p className="text-sm">record: {fighter.record}</p>
        </div>
    );
};

export default Fighter;

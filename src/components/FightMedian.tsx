import React from "react";

interface FightMedianProps {
    odds: string
}
const FightMedian = (props : FightMedianProps) => {
    return (
        <div className="p-4 grow-0">
            <div className="flex justify-center">
                <h3 className="text-xs">{props.odds}</h3>
            </div>
        </div>
    );
};

export default FightMedian;

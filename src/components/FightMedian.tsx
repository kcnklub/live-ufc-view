import React from "react";

interface FightMedianProps {
    odds: string
}
const FightMedian = (props : FightMedianProps) => {
    return (
        <div className="p-4 grow-0">
            <div className="flex justify-center content-center pt-4">
                <p className=" text-xs">{props.odds}</p>
            </div>
        </div>
    );
};

export default FightMedian;

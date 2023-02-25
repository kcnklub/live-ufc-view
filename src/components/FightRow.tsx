import React from "react";
import Fighter from "./Fighter";

interface RowData {
    left: Fighter;
    right: Fighter;
}

const FightRow = (props: RowData) => {
    return (
        <div className="grid grid-cols-2">
            <Fighter side="left" fighter={props.left}></Fighter>
            <Fighter side="right" fighter={props.right}></Fighter>
        </div>
    );
};

export default FightRow;

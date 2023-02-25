import React from "react";
import Fighter from "./Fighter";

interface RowData {
    left: Fighter;
    right: Fighter;
}

const FightRow = (props: RowData) => {
    return (
        <div className="grid grid-cols-2">
            <Fighter fighter={props.left}></Fighter>
            <Fighter fighter={props.right}></Fighter>
        </div>
    );
};

export default FightRow;

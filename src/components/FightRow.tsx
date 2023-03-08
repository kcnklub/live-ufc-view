import React from "react";
import Fighter from "./Fighter";
import FightMedian from "./FightMedian";
import styles from "./FightRow.module.css";

interface RowData {
    left: Fighter;
    right: Fighter; 
    odds: string
}

const FightRow = (props: RowData) => {
    return (
        <div className={`${styles.row} grid grid-cols-7`}>
            <Fighter side="left" fighter={props.left}></Fighter>
            <FightMedian odds={props.odds}></FightMedian>
            <Fighter side="right" fighter={props.right}></Fighter>
        </div>
    );
};

export default FightRow

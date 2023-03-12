import React from "react";
import Fighter from "./Fighter";
import FightMedian from "./FightMedian";
import StatBox from "./StatBox";
import styles from "./FightRow.module.css";

interface RowData {
    left: Fighter;
    right: Fighter; 
    odds: string;
}

const FightRow = (props: RowData) => {
    const hasStats : boolean = props.left.stats.kd !== "";

    const borderRadius = () => {
        return hasStats ? "rounded-t-lg" : "rounded-lg";
    }
    return (
    <>
        <div className={`${styles.row} ${borderRadius()} grid grid-cols-7`}>
            <Fighter side="left" fighter={props.left}/>
            <FightMedian odds={props.odds}/>
            <Fighter side="right" fighter={props.right}/>
        </div>
        {
            hasStats &&
            <StatBox 
                leftStats={props.left.stats}
                rightStats={props.right.stats}/>
        }
    </>
    );
};

export default FightRow

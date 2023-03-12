import React from "react"
import styles from "./StatBox.module.css";

interface StatsProp {
    leftStats: FighterStats; 
    rightStats: FighterStats; 
}

const StatBox = (props: StatsProp) => {

    return (
            <ul className={`${styles.stats} `}>
                <StatRow 
                    leftStat={props.leftStats.kd} 
                    rightStat={props.rightStats.kd} 
                    statName="KD"/>
                <StatRow 
                    leftStat={props.leftStats.total_strikes} 
                    rightStat={props.rightStats.total_strikes} 
                    statName="TOTAL STRIKES"/>
                <StatRow 
                    leftStat={props.leftStats.sig_strikes} 
                    rightStat={props.rightStats.sig_strikes} 
                    statName="SIG STRIKES"/>
                <StatRow 
                    leftStat={props.leftStats.head} 
                    rightStat={props.rightStats.head} 
                    statName="HEAD"/>
                <StatRow 
                    leftStat={props.leftStats.body} 
                    rightStat={props.rightStats.body} 
                    statName="BODY"/>
                <StatRow 
                    leftStat={props.leftStats.legs} 
                    rightStat={props.rightStats.legs} 
                    statName="LEGS"/>
                <StatRow 
                    leftStat={props.leftStats.control} 
                    rightStat={props.rightStats.control} 
                    statName="CONTROL"/>
                <StatRow 
                    leftStat={props.leftStats.take_downs} 
                    rightStat={props.rightStats.take_downs} 
                    statName="TAKE DOWNS"/>
                <StatRow 
                    leftStat={props.leftStats.sub_att} 
                    rightStat={props.rightStats.sub_att} 
                    statName="SUB ATT"/>
            </ul>
    )
}

interface StatRowProps{
    leftStat: string;
    rightStat: string;
    statName: string;
}

const StatRow = (props: StatRowProps) => {
    return (
        <li className="grid grid-cols-7">
            <p className="col-span-3 flex justify-end">{props.leftStat}</p>
            <div className="pt-2 grow-0">
                <p className="text-xs flex justify-center">{props.statName}</p>
            </div>
            <p>{props.rightStat}</p>
        </li>
    )
}

export default StatBox

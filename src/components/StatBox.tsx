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
                    leftStat={props.leftStats.height}
                    rightStat={props.rightStats.height}
                    statName="HEIGHT"/>
                <StatRow 
                    leftStat={props.leftStats.weight}
                    rightStat={props.rightStats.weight}
                    statName="WEIGHT"/>
                <StatRow 
                    leftStat={props.leftStats.reach}
                    rightStat={props.rightStats.reach}
                    statName="REACH"/>
                <StatRow 
                    leftStat={props.leftStats.stance}
                    rightStat={props.rightStats.stance}
                    statName="STANCE"/>
                <StatRow 
                    leftStat={props.leftStats.sig_str_lpm}
                    rightStat={props.rightStats.sig_str_lpm}
                    statName="SIG STR LPM"/>
                <StatRow 
                    leftStat={props.leftStats.sig_str_acc}
                    rightStat={props.rightStats.sig_str_acc}
                    statName="SIG STR ACC"/>
                <StatRow 
                    leftStat={props.leftStats.td_avg}
                    rightStat={props.rightStats.td_avg}
                    statName="TD AVG"/>
                <StatRow 
                    leftStat={props.leftStats.td_acc}
                    rightStat={props.rightStats.td_acc}
                    statName="TD ACC"/>
                <StatRow 
                    leftStat={props.leftStats.sub_avg}
                    rightStat={props.rightStats.sub_avg}
                    statName="SUB AVG"/>


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
    const hasData: boolean = props.leftStat !== ""
    return (<>
        {
            hasData && 
            <li className="grid grid-cols-7">
                <p className="col-span-3 flex justify-end">{props.leftStat}</p>
                <div className="pt-2 grow-0">
                    <p className="text-xs flex justify-center">{props.statName}</p>
                </div>
                <p>{props.rightStat}</p>
            </li>
        }</>
    )
}

export default StatBox

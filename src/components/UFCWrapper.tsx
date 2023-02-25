import React from "react";
import FightRow from "./FightRow";

interface UFCCardData {
    fights: Fight[];
}

const UFCWrapper = (data: UFCCardData) => {
    return (
        <>
            <div>
                {data.fights.map((fight: Fight) => {
                    return (
                        <FightRow
                            key={fight.id}
                            left={fight.left_fighter}
                            right={fight.right_fighter}
                        ></FightRow>
                    );
                })}
            </div>
        </>
    );
};

export default UFCWrapper;

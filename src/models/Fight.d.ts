interface CardProps {
    title: string;
    fights: Fight[]
}

interface FightCard {
    name: string;
    fights: Fight[];
}

interface Fight {
    id: string;
    left_fighter: Fighter;
    right_fighter: Fighter;
    odds: string;
}

interface Fighter {
    name: string;
    record: string;
    headshotSource: string;
    stats: FighterStats;
}

interface FighterStats {
    kd: string; 
    total_strikes: string;
    sig_strikes: string;
    head: string;
    body: string; 
    legs: string;
    control: string; 
    take_downs: string; 
    sub_att: string;
}

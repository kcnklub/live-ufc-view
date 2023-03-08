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
}

interface FighterStats {
    height: string, 
    weight: string, 
    reach: string, 
    stance: string, 
    sig_str_lpm: string, 
    sig_str_acc: string, 
    td_avg: string, 
    td_acc: string, 
    sub_avg: string
}

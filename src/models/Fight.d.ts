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
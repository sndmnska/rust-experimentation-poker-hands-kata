#![allow(dead_code)]

// mod structenum;

fn main() {
    println!("Hello, world!");
}

struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Eq)]

struct Card {
    suit: Suit,
    rank: Rank,
}

fn parse_card(s: &str) -> Card {
    let mut chars = s.chars();
    let rank = chars.next().unwrap();
    let suit = chars.next().unwrap();

    let rank: Rank = match rank {
        '2' => Rank::Two,
        '3' => Rank::Three,
        '4' => Rank::Four,
        '5' => Rank::Five,
        '6' => Rank::Six,
        '7' => Rank::Seven,
        '8' => Rank::Eight,
        '9' => Rank::Nine,
        'T' => Rank::Ten,
        'J' => Rank::Jack,
        'Q' => Rank::Queen,
        'K' => Rank::King,
        'A' => Rank::Ace,
        _ => panic!("Invalid rank"),
    };

    let suit: Suit = match suit {
        'C' => Suit::Clubs,
        'D' => Suit::Diamonds,
        'H' => Suit::Hearts,
        'S' => Suit::Spades,
        _ => panic!("Invalid suit"),
    };

    Card { suit, rank } // a shortcut for "suit: suit", for example. ("field: variable")
}

// "2H 3D 5S 9C KD" &str
// "2H"
fn parse_hand(s: &str) -> Hand {
    // let mut cards = Vec::new();
    // for card_str in s.split_whitespace() {
    //     cards.push(parse_card(card_str));
    // }

    // let whitespace_split = s.split_whitespace();
    // let cards_map = whitespace_split.map(parse_card);
    // let cards = cards_map.collect();

    let mut cards = s.split_whitespace().map(parse_card).collect::<Vec<_>>();
    cards.sort_by_key(|card| card.rank);

    // let cards = vec![
    //     parse_card(&s[0..2]),
    //     parse_card(&s[3..5]),
    //     parse_card(&s[6..8]),
    //     parse_card(&s[9..11]),
    //     parse_card(&s[12..14]),
    // ];

    // let mut cards = Vec::new();
    // cards.push(parse_card(&s[0..2]));
    // cards.push(parse_card(&s[3..5]));
    // cards.push(parse_card(&s[6..8]));
    // cards.push(parse_card(&s[9..11]));
    // cards.push(parse_card(&s[12..14]));
    // cards.push(parse_card("2H"));

    Hand { cards }
}

#[derive(Debug, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Player1,
    Player2,
    Draw,
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Outcome {
    Outcome::Player2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_poker_hands() {
        // Example hand:
        //
        // 2H 3D 5S 9C KD   2C 3H 4S 8C AH
        // Player 2 wins.
        let hand1 = parse_hand("2H 3D 5S 9C KD");
        let hand2 = parse_hand("2C 3H 4S 8C AH");
        assert_eq!(compare_hands(&hand1, &hand2), Outcome::Player2);
    }

    // #[test]
    // fn test_compare_poker_hands2() {
    //     // Example hand:
    //     //
    //     // 2H 3D 5S 9C KD   2C 3H 4S 8C AH
    //     // Player 2 wins.
    //     let hand1 = parse_hand("2C 3H 4S 8C AH");
    //     let hand2 = parse_hand("2H 3D 5S 9C KD");
    //     assert_eq!(compare_hands(&hand1, &hand2), Outcome::Player1);
    // }

    #[test]
    fn test_parse_card_() {
        let card: Card = parse_card("2H");
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.rank, Rank::Two);
    }

    #[test]
    fn test_parse_card_king_of_diamonds() {
        let card: Card = parse_card("KD");
        assert_eq!(card.suit, Suit::Diamonds);
        assert_eq!(card.rank, Rank::King);
    }

    #[test]
    fn test_parse_hand_five_cards() {
        let hand: Hand = parse_hand("2H 3D 5S 9C KD");
        assert_eq!(
            hand.cards,
            [
                parse_card("2H"),
                parse_card("3D"),
                parse_card("5S"),
                parse_card("9C"),
                parse_card("KD")
            ]
        );
    }

    #[test]
    fn test_parse_hand_five_cards_different_order() {
        let hand: Hand = parse_hand("3D 2H 5S KD 9C");
        assert_eq!(
            hand.cards,
            [
                parse_card("2H"),
                parse_card("3D"),
                parse_card("5S"),
                parse_card("9C"),
                parse_card("KD")
            ]
        );
    }

    #[test]
    fn test_compare_two_ranks() {
        let rank1 = Rank::Ace;
        let rank2 = Rank::King;
        assert!(rank1 > rank2);
    }

    #[test]
    fn test_compare_two_ranks_reversed() {
        let rank1 = Rank::King;
        let rank2 = Rank::Ace;
        assert!(rank1 < rank2);
    }

    #[test]
    fn test_compare_two_ranks_equal() {
        let rank1 = Rank::Ace;
        let rank2 = Rank::Ace;
        assert!(rank1 == rank2);
    }

    // #[test]
    // fn test_compare_two_ranked_cards() {
    //     let card_1: Card = parse_card("KD");
    //     let card_2: Card = parse_card("QD");

    //     assert!(card_1 > card_2);
    // }

    // #[test]
    // fn test_compare_two_ranked_cards_equal() {
    //     let card_1: Card = parse_card("KD");
    //     let card_2: Card = parse_card("KH");

    //     assert!(card_1 == card_2);
    // }
}

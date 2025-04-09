#![allow(dead_code)]

mod structenum;

fn main() {
    println!("Hello, world!");
}

struct Hand {}

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

fn parse_hand(s: &str) -> Hand {
    Hand {}
}

#[derive(Debug, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Eq)]
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

    #[test]
    fn test_parse_card() {
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
}

#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;

// mod structenum;

fn main() {
    println!("Hello, world!");
}

struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, Eq, PartialEq)]

struct Card {
    suit: Suit,
    rank: Rank,
}

// impl PartialEq for Card {
//     fn eq(&self, other: &Self) -> bool {
//         self.rank == other.rank
//     }
// }

// impl Ord for Card {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.rank.cmp(&other.rank)
//     }
// }

// impl PartialOrd for Card {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

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
    cards.reverse(); // sort from A to 2, opposite order from above

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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    // Check for pairs
    //
    let is_pair_h1 = detect_pair(hand1);
    let is_pair_h2 = detect_pair(hand2);

    // if is_pair_h1 && is_pair_h2 {
    // } else if is_pair_h1 {
    //     return Ordering::Greater;
    // } else if is_pair_h2 {
    //     return Ordering::Less;
    // } else {
    //     //
    // }

    // TODO Write a test to check what happens if there's three of a kind on this....
    // TODO Eventually end up with a bunch of functions check for these things

    match (is_pair_h1, is_pair_h2) {
        (true, true) => {
            // TODO compare the card in the pair???

            // TODO if that card is the same then compare the remaining
            // cards for high card?
        }
        (true, false) => {
            return Ordering::Greater;
        }
        (false, true) => {
            return Ordering::Less;
        }
        _ => {}
    }

    let ranks1 = hand1.cards.iter().map(|card| card.rank).collect::<Vec<_>>();
    let ranks2 = hand2.cards.iter().map(|card| card.rank).collect::<Vec<_>>();

    ranks1.cmp(&ranks2) // returns result of comparison (as an Ordering)

    // What the above does:
    // if ranks1 > ranks2 {
    //     Outcome::Player1
    // } else if ranks1 < ranks2 {
    //     Outcome::Player2
    // } else {
    //     Outcome::Draw
    // }
}

fn detect_pair(hand: &Hand) -> bool {
    // TODO is there a pair, and option of the rank coming through, and
    // returning a vector of the cards not in the pair

    let mut counts: HashMap<Rank, u8> = HashMap::new();

    // do the counting here
    for card in &hand.cards {
        let rank = card.rank;
        counts
            .entry(rank)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        // let count = counts.get(&rank);
        // if let Some(count) = count {
        //     counts.insert(rank, count + 1);
        // } else {
        //     counts.insert(rank, 1);
        // }
    }

    for (_rank, count) in counts {
        if count == 2 {
            return true;
        }
    }
    false
    // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_poker_hands() {
        // Hand 2 wins.
        let hand1 = parse_hand("2H 3D 5S 9C KD");
        let hand2 = parse_hand("2C 3H 4S 8C AH");
        assert_eq!(compare_hands(&hand1, &hand2), Ordering::Less);
    }

    #[test]
    fn test_compare_poker_hands2() {
        // Hand 1 wins.
        let hand1 = parse_hand("2C 3H 4S 8C AH");
        let hand2 = parse_hand("2H 3D 5S 9C KD");
        assert_eq!(compare_hands(&hand1, &hand2), Ordering::Greater);
    }

    #[test]
    fn test_compare_poker_hands_draw() {
        // Draw
        let hand1 = parse_hand("2C 3H 4S 8C AH");
        let hand2 = parse_hand("2H 3C 4H 8S AC");
        assert_eq!(compare_hands(&hand1, &hand2), Ordering::Equal);
    }

    #[test]
    fn test_compare_poker_hands_second_high_card() {
        // Hand 2 wins
        let hand1 = parse_hand("2C 3H 4S 8C AH");
        let hand2 = parse_hand("2H 3D 5S 9C AD");
        assert_eq!(compare_hands(&hand1, &hand2), Ordering::Less);
    }

    #[test]
    fn test_detect_pair_true() {
        let hand: Hand = parse_hand("2C 3H 4S 8C 8H");
        assert!(detect_pair(&hand));
    }

    #[test]
    fn test_detect_pair_false() {
        let hand: Hand = parse_hand("2C 3H 4S 8C 9H");
        assert!(!detect_pair(&hand));
    }

    #[test]
    fn test_pair_poker_hand1() {
        // Hand 1 wins
        let hand1: Hand = parse_hand("2C 3H 4S 8C 8H");
        let hand2: Hand = parse_hand("2H 3D 5S 9H KH");
        assert_eq!(compare_hands(&hand1, &hand2), Ordering::Greater);
    }

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
                parse_card("KD"),
                parse_card("9C"),
                parse_card("5S"),
                parse_card("3D"),
                parse_card("2H")
            ]
        );
    }

    #[test]
    fn test_parse_hand_five_cards_different_order() {
        let hand: Hand = parse_hand("3D 2H 5S KD 9C");
        assert_eq!(
            hand.cards,
            [
                parse_card("KD"),
                parse_card("9C"),
                parse_card("5S"),
                parse_card("3D"),
                parse_card("2H")
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

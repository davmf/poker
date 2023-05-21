#![allow(unused)]

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

use std::{cmp::Ordering, ops::Index};

trait SortedImpl {
    fn sorted(self) -> Self;
}

impl<E> SortedImpl for Vec<E>
    where E: std::cmp::Ord 
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {hands:?}, which hand wins?")
}

enum PokerHandType {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq, Clone)]
struct Card {
    value: String,
    suit: String,
}

const CARDS: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
const SUITS: [&str; 4] = ["H", "D", "S", "C"];

impl Card {
    fn new(card: &str) -> Result<Card, String> {
        let values: Vec<String> = 
            CARDS.iter()
                .map(|c| c.to_string())
                .collect();

        let suits: Vec<String> = SUITS.iter().map(|c| c.to_string()).collect();
        let value = card[..card.len()-1].to_string();
        let suit = card[card.len()-1..card.len()].to_string();

        if !values.contains(&value) || !suits.contains(&suit) {
            return Err("Invalid card!".to_string())
        }
        Ok( Card { value, suit } )
    }

    fn suit(&self) -> String {
        self.suit.clone()
    }

    fn numeric_value(&self) -> i8 {
        CARDS.iter().position(|s| *s == self.value).unwrap() as i8 + 2
    }

}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let index_self = CARDS
            .iter()
            .position(|&x| x == &self.value)
            .unwrap();
        let index_other = CARDS
            .iter()
            .position(|&x| x == &other.value)
            .unwrap();
        index_self.cmp(&index_other)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct PokerHand {
    cards: Vec<Card>,
}

impl PokerHand {
    fn new(hand: &str) -> Result<PokerHand, String> {
        let card_strings: Vec<String> =
            hand
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        if card_strings.len() != 5 {
            return Err("Invalid length".to_string())
        }

        let cards = card_strings
            .iter()
            .map(|card_string| Card::new(card_string))
            .collect::<Result<Vec<Card>, _>>()?;   
    
        Ok( PokerHand { cards })
    }

    fn is_flush(&self) -> bool {
        let hand_suits: Vec<String> = self.cards
            .iter()
            .map(|card| card.suit())
            .collect();

        Self::all_elements_equal(&hand_suits)

    }

    fn is_straight_flush(&self) -> bool {        
        self.is_flush() && self.is_straight()
    }

    fn all_elements_equal<T: PartialEq>(slice: &[T]) -> bool {
        if let Some((first, rest)) = slice.split_first() {
            rest.iter().all(|x| x == first)
        } else {
            true
        }
    }

    fn is_straight(&self) -> bool {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();
    
        for i in 0..sorted_cards.len() - 1 {
            if sorted_cards[i].numeric_value() + 1 != sorted_cards[i + 1].numeric_value() {
                return false;
            }
        }
    
        true
    }   

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_value() {
        let card = Card::new("2C").unwrap();
        assert!(card.numeric_value() == 2);
    }

    #[test]
    fn test_new_card() {
        let card = Card::new("2C").unwrap();
        assert!(card.value == "2");
        assert!(card.suit == "C");
    }

    #[test]
    fn test_another_new_card() {
        let card = Card::new("10D").unwrap();
        assert!(card.value == "10");
        assert!(card.suit == "D");
    }

    #[test]
    fn test_bad_card() {
        let card = Card::new("11S");
        assert!(card.is_err());
    }

    #[test]
    fn test_card_sorting() {
        let card_names = ["10H", "6C", "2D", "KS", "JH"];
        let sorted_card_names = ["2D", "6C", "10H", "JH", "KS"];
        let cards: Vec<Card> = card_names.iter().map(|cn| Card::new(&cn).unwrap()).collect::<Vec<Card>>().sorted();
        let sorted_cards: Vec<Card> = sorted_card_names.iter().map(|cn| Card::new(&cn).unwrap()).collect();
        assert!(cards == sorted_cards);
    }

    #[test]
    fn test_poker_hand() {
        let hand = PokerHand::new("2C 3D 4H 5C 8S");
        assert!(hand.is_ok());
    }

    #[test]
    fn test_bad_poker_hand() {
        let hand = PokerHand::new("2C 3X 4H 5C 8S");
        assert!(hand.is_err());
    }

    #[test]
    fn test_is_flush() {
        let hand = PokerHand::new("2H 3H 4H 5H 8H").unwrap();
        assert!(hand.is_flush());
        let hand = PokerHand::new("2D 3D 6D 7D 8D").unwrap();
        assert!(hand.is_flush());
        let hand = PokerHand::new("2C 3C 6C 7C 8C").unwrap();
        assert!(hand.is_flush());
        let hand = PokerHand::new("2S 3S 6S 7S 8S").unwrap();
        assert!(hand.is_flush());
        let hand = PokerHand::new("2S 3C 6S 7S 8S").unwrap();
        assert!(!hand.is_flush());
    }

    #[test]
    fn test_is_straight() {
        let hand = PokerHand::new("2H 3D 4C 5H 6S").unwrap();
        assert!(hand.is_straight());
        let hand = PokerHand::new("2D 3D 6C 5D 4S").unwrap();
        assert!(hand.is_straight());
        let hand = PokerHand::new("2D 3D 8C 5D 4S").unwrap();
        assert!(!hand.is_straight());
    }

    #[test]
    fn test_is_straight_flush() {
        let hand = PokerHand::new("2D 3D 4D 5D 6D").unwrap();
        assert!(hand.is_straight_flush());
        let hand = PokerHand::new("9C 8C 10C QC JC").unwrap();
        assert!(hand.is_straight_flush());
        let hand = PokerHand::new("2D 3D 6C 5D 4S").unwrap();
        assert!(!hand.is_straight_flush());
        let hand = PokerHand::new("2D 3D 8D 5D 4D").unwrap();
        assert!(!hand.is_straight_flush());
    }
}

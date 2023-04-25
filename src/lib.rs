/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

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
    StraightFlush(),
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq)]
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
        if hand_suits.iter().filter(|suit| *suit == "H").count() == 4 {
            return true;
        }
        else if hand_suits.iter().filter(|suit| *suit == "D").count() == 4 {
            return true;
        }
        else if hand_suits.iter().filter(|suit| *suit == "C").count() == 4 {
            return true;
        }
        else if hand_suits.iter().filter(|suit| *suit == "S").count() == 4 {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

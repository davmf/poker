/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

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

struct Card {
    value: String,
    suit: String,
}

impl Card {
    fn new(card: &str) -> Result<Card, String> {
        let values: Vec<String> = 
            vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"]
                .iter()
                .map(|c| c.to_string())
                .collect();

        let suits: Vec<String> = vec!["C", "S", "D", "H"].iter().map(|c| c.to_string()).collect();
        let value = card[..card.len()-1].to_string();
        let suit = card[card.len()-1..card.len()].to_string();

        if !values.contains(&value) || !suits.contains(&suit) {
            return Err("Invalid card!".to_string())
        }
        Ok( Card { value, suit } )
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

    fn is_straight_flush(&self) {

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

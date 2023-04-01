/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {hands:?}, which hand wins?")
}

struct Card {
    value: char,
    suit: char,
}

impl Card {
    fn new(value: char, suit: char) -> Result<Card> {
        let values = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let suits = vec!["C", "S", "D", "H"];
        
        if value in ['A', '2', '3', '4', '5', '6', '7', '8', '9']
    };
    Ok(Card{ value, suit};)
}

struct PokerHand {
    cards: Vec<Card>,
}

impl PokerHand {
    fn new(hand: &str) -> Result<PokerHand> {

}
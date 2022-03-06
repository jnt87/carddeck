mod poker;

use poker::*;

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(2);
        Self { cards }
    }

    fn draw(&mut self, card: Option<Card>) {
        let c = match card {
            Some(top) => top,
            None => {
                panic!("No card!")
            }
        };
        if self.cards.len() < 2 {
            self.cards.push(c);
        }
    }
}

struct River {
    cards: Vec<Card>,
}

impl River {
    fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(3);
        Self { cards }
    }

    fn draw(&mut self, card: Option<Card>) {
        let c = match card {
            Some(top) => top,
            None => {
                panic!("No card!")
            }
        };
        if self.cards.len() < 3 {
            self.cards.push(c);
        }
    }
}


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let mut hand = Hand::new();
    let mut river = River::new();
    hand.draw(deck.draw());
    hand.draw(deck.draw());
    river.draw(deck.draw());
    river.draw(deck.draw());
    river.draw(deck.draw());
    let card = hand.cards.get(1).unwrap();
    println!("Hello, world! {} of {}", card.val, card.st);
}

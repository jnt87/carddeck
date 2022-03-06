use std::fmt;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Eq, Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Suit {
    HEARTS = 1, 
    SPADES = 2, 
    DIAMONDS = 3, 
    CLUBS = 4,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::HEARTS => write!(f, "Hearts"),
            Suit::SPADES => write!(f, "Spades"),
            Suit::DIAMONDS => write!(f, "Diamonds"),
            Suit::CLUBS => write!(f, "Clubs"),
        }
    }
}

impl Suit {
    fn from_i32(value: i32) -> Suit {
        match value {
            1 => Suit::HEARTS,
            2 => Suit::SPADES,
            3 => Suit::DIAMONDS,
            4 => Suit::CLUBS,
            _ => panic!("Unkown Suit value: {}", value),
        }
    }
}

#[derive(Eq, Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Face {
    ACE = 1, 
    TWO = 2, 
    THREE = 3, 
    FOUR = 4, 
    FIVE = 5, 
    SIX = 6, 
    SEVEN =7, 
    EIGHT = 8, 
    NINE = 9, 
    TEN = 10, 
    JACK = 11, 
    QUEEN = 12, 
    KING = 13,
    ACE_HIGH = 14,
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        match *self {
            Face::ACE => write!(f, "Ace"),
            Face::TWO => write!(f, "Two"),
            Face::THREE => write!(f, "Three"),
            Face::FOUR => write!(f, "Four"),
            Face::FIVE => write!(f, "Five"),
            Face::SIX => write!(f, "Six"),
            Face::SEVEN => write!(f, "Seven"),
            Face::EIGHT => write!(f, "Eight"),
            Face::NINE => write!(f, "Nine"),
            Face::TEN => write!(f, "Ten"),
            Face::JACK => write!(f, "Jack"),
            Face::QUEEN => write!(f, "Queen"),
            Face::KING => write!(f, "King"),
            Face::ACE_HIGH => write!(f, "Ace"),
        }
    }
}

impl Face {
    fn from_i32(value: i32) -> Face {
        match value {
            1 => Face::ACE,
            2 => Face::TWO,
            3 => Face::THREE,
            4 => Face::FOUR,
            5 => Face::FIVE,
            6 => Face::SIX,
            7 => Face::SEVEN,
            8 => Face::EIGHT,
            9 => Face::NINE,
            10 => Face::TEN,
            11 => Face::JACK,
            12 => Face::QUEEN,
            13 => Face::KING,
            14 => Face::ACE_HIGH,
            _ => panic!("Unknown Face value: {}", value),
        }
    }
}

#[derive(Eq, Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Card {
    pub st: Suit,
    pub val: Face,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.val as i32) < (other.val as i32) {
            Ordering::Less
        } else if (self.val as i32) > (other.val as i32) {
            Ordering::Greater
        } else {
            if (self.st as i32) > (other.st as i32) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}


impl Card {
    fn new(st: Suit, val: Face) -> Self {
        Self { st, val}
    }

    fn new_fromi32(suit: i32, face: i32) -> Self {
        let st = Suit::from_i32(suit);
        let val = Face::from_i32(face);
        Self { st, val }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(ace_high: bool) -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        for i in 1..5 {
            for j in 1..14 {
                if (j == 1) & ace_high {
                    cards.push(Card::new_fromi32(i, 14));
                }
                cards.push(Card::new_fromi32(i, j));
            }
        }
        Self { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        if self.cards.len() == 0 {
            for i in 1..5 {
                for j in 1..14 {
                    self.cards.push(Card::new_fromi32(i, j));
                }
            }
            self.shuffle(); // mess with people counting cards
            println!("New Deck!"); // probably dont need this
        }
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut index = self.cards.len();
        if index == 0 {
            for i in 1..5 {
                for j in 1..14 {
                    self.cards.push(Card::new_fromi32(i, j));
                }
            }            
        }
        index = self.cards.len()-1;
        while index > 0 {
            let to_swap = rand::thread_rng().gen_range(0..index);
            self.cards.swap(to_swap, index);
            index = index - 1;
        }
    }
}

/*
fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let card = match deck.draw() {
        Some(top) => top,
        None => {
            panic!("No card!")
        }
    };
    println!("Hello, world! {} of {}", card.val, card.st);
}*/

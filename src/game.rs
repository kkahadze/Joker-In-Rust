use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::card;
use crate::card::{Card, Suit, Rank};
use crate::user;
use std::convert::TryInto;
use std::io;
use std::fmt::Debug;
use std::io::stderr;
use std::cmp;
use std::thread::current;
use std::fmt;
pub struct JokerDeck(Vec<Card>);

impl JokerDeck {
    pub fn new() -> JokerDeck {
        let mut cards:Vec<Card> = Vec::with_capacity(36);
        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for rank in [Rank::Ace, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Joker] {
                    if rank == Rank::Six && (suit == Suit::Spades || suit == Suit::Clubs) {
                        cards.push(Card::new(Rank::Joker, suit.clone()));
                    } else {
                        cards.push(Card::new(rank.clone(), suit.clone()));
                    }
            }
        }
        let ref mut rng = thread_rng();
        cards.shuffle(rng);
        JokerDeck(cards)
    }

    pub fn empty() -> JokerDeck {
        let mut cards:Vec<Card> = Vec::with_capacity(36);
        JokerDeck(cards)
    }

    pub fn filter_playable(&self, wildsuit : Option<Suit>, first_suit : Option<Suit>) -> JokerDeck {
        match &first_suit {
            Some(first) => {
                let suit_copy = first.clone();
                let has_first_suit: bool = self.0.iter().fold(true, |sum: bool, val| sum && val.is_suit(suit_copy));
                
                if has_first_suit {
                    JokerDeck(self.0.clone().into_iter().filter(|card| card.is_joker() || card.is_suit(suit_copy)).collect())
                } else {
                    match wildsuit {
                        Some(wild   )   => {
                            let has_wild_suit: bool = self.0.iter().fold(true, |sum: bool, val| sum && val.is_suit(wild));
                            if has_wild_suit{
                                JokerDeck(self.0.clone().into_iter().filter(|card| card.is_joker() || card.is_suit(wild)).collect()) 
                            } else {
                                JokerDeck(self.0.clone())
                            }
                        },
                        None                 =>  JokerDeck(self.0.clone()),
                    }
                }
                // if self.0.into_iter().filter(|card: Card| card.is_suit(suit_copy)){ if the use doesnt have the first suit
                    // ()
                // }
            },
            None            => JokerDeck(self.0.clone()),
        }
    }

    fn get_length(&self) -> usize {
        self.0.len()
    }

    fn get(&self, index: u16) -> Option<Card>{
        let card = self.0.get(index as usize).expect("INDEX OUT OF BOUNDS ERROR");
        match self.0.get(index as usize){
            Some(this_card) => Some(this_card.clone()),
            None                  => None,
        }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng())
    }

    pub fn count(&self) -> u16 {
        self.0.len() as u16
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn push(&mut self, card: Card){
        self.0.push(card);
    }

    pub fn peek(&self) -> Option<Card> {
        match self.0.last(){
            Some(card)   => Some(card.copy()),
            None               => None,
        }
    }
}

impl fmt::Display for JokerDeck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut all = String::from("");
        for i in 0..(self.0.len()) {
            all.push_str(&format!("{}  ", self.0[i]));
        }
        write!(f, "{}", all)
    }
}

pub struct User{
    pub id:     u16,
    pub cards:  JokerDeck,
    score:      u16,
    called:     u16,
    taken:      u16,
    dealer:     bool,
}

impl User{
    pub fn new(id_in : u16) -> User {
        User{
            id :   id_in,
            cards: JokerDeck::empty(),
            score: 0,
            called:0,
            taken: 0,
            dealer: false,
        }
    }

    pub fn add_card(&mut self, card: Card){
        self.cards.push(card);
    }
}

pub struct Game{
    pub deck:           JokerDeck,
    pub wildcard:       Option<Card>,
    pub first_suit:     Option<Suit>,
    pub cards_dealt:    u16,
    pub round:          u16, // could be round struct, table could own cards, each player would have option of a card, who won round would be a method of round struct
    pub play:           u16,
    pub users:          Vec<User>,
    pub dealer:         u16,
}

impl Game{
    pub fn new() -> Game {
        let mut deck_in = JokerDeck::new();
        deck_in.shuffle();
        deck_in.shuffle();
        let mut rng = rand::thread_rng();
        let rand_0_to_3 = rng.gen::<u16>() % 4;
        let users_in = vec![User::new(0), User::new(1), User::new(2), User::new(3)];

        Game{
            deck:           deck_in,
            wildcard:       None,
            first_suit:     None,
            cards_dealt:    0,
            round:          0,
            play:           0,
            users:          users_in,
            dealer:         rand_0_to_3,
        }
    }

    fn get_wildsuit(&self) -> Option<Suit> {
        match self.wildcard {
            Some(card)     => {
                if card.is_joker() {
                    None
                } else {
                    Some(card.suit)
                }
            },
            None                => {
                println!("get_wildsuit ERROR: ");
                None
            },
        }
    }

    pub fn next_dealer(&mut self) -> u16 {
        (self.dealer + 1) % 4
    }

    pub fn set_dealer(&mut self) {
        self.dealer = Game::next_dealer(self);
    }

    pub fn update_round(&mut self) -> bool{
        let round: u16 = self.round;
        let play: u16 = self.play;

        self.set_dealer();

        if round  == 4 && play == 4 {
            false
        } else if play == 8 && (round == 1 || round == 3){
            self.play = 1;
            self.round += 1;
            self.cards_dealt = self.play;
            true
        } else if play == 4 && round == 2 {
            self.round = 3;
            self.play = 1;
            self.cards_dealt = self.play;
            true
        } else if self.round == 0 && self.play == 0 {
            self.round = 1;
            self.play = 1;
            self.cards_dealt = self.play;
            true
        } else {
            self.play += 1;
            self.cards_dealt = self.play;
            true
        }
    } 

    fn set_calls(&mut self, u0 : u16, u1 : u16, u2: u16, u3: u16){
        self.users[0].called = u0;
        self.users[1].called = u1;
        self.users[2].called = u2;
        self.users[3].called = u3;
    }

    pub fn ask_set_calls(&mut self) -> Result<(), std::fmt::Error> { 
        let mut calls: Vec<u16> = vec![0,0,0,0];
        for i in 1..5{
            let call: u16;
            if i == 4 {
                call = user::get_last_call(self.cards_dealt, calls.iter().sum::<u16>() as u16, (i + self.dealer) % 4);
            } else {
                call = user::get_call((i + self.dealer) % 4)
            }
            calls[(i + self.dealer) as usize % 4] = call;
        }
        Ok(())
    }

    pub fn deal_to_all_users(&mut self){
        for i in 1..5 {
            self.users[((self.dealer + i) % 4) as usize].add_card(self.deck.pop().unwrap());
        }
        match self.deck.peek(){
            Some(card)  => {
                self.wildcard = Some(card);
            },
            None        => self.wildcard = Some(Card::new(Rank::Joker, Suit::Hearts)),
        }
    }

    pub fn compute_score(&self, user : &User) -> i16 {
        let called = user.called;
        let taken = user.taken;
        
        if called == 0 && taken == 0{ 
            50
        } else if taken == 0{
            -200
        } else if taken == self.cards_dealt {
            (self.cards_dealt * 100) as i16
        } else if taken != called {
            (taken * 10) as i16
        } else { 
            (taken * 50 + 50) as i16
        }
    }

    fn card_to_weight(&mut self, card: &Card, user : u16) -> u16{
        let mut counter = 0;

        let mut wild = Suit::Clubs; // This value should always be changed
        let mut first = Suit::Clubs; // Should be changed

        match self.wildcard {
            Some(card) => {
                match card.suit{
                    Suit::Clubs => wild = Suit::Clubs,
                    Suit::Diamonds => wild = Suit::Diamonds,
                    Suit::Hearts => wild = Suit::Hearts,
                    Suit::Spades => wild = Suit::Spades,
                }
            }
            None            => println!("WILDCARD ERROR"),
        }

        if  card.suit == wild {
            counter += 200;
        } else if (self.dealer + 1) % 4 == user {
            counter += 100;
        }

        match card.rank {
            Rank::Six      => (),
            Rank::Seven    => counter += 1,
            Rank::Eight    => counter += 2,
            Rank::Nine     => counter += 3,
            Rank::Ten      => counter += 4,
            Rank::Jack     => counter += 5,
            Rank::Queen    => counter += 6,
            Rank::King     => counter += 7,
            Rank::Ace      => counter += 8,
            Rank::Joker    => counter = 300,
        }
        counter
    }

    pub fn compute_winner(&mut self, c0: &Card, c1: &Card, c2: &Card, c3: &Card) -> u16 {
        let mut winner = 4; // This should never return
        let zero_one;
        if self.card_to_weight(c0, 0) > self.card_to_weight(c1, 1){
            zero_one = 0;
        } else {
            zero_one = 1;
        }

        if self.card_to_weight(c2, 2) > self.card_to_weight(c3, 3){
            if zero_one == 0 {
                if self.card_to_weight(c2, 2) > self.card_to_weight(c0, 0){
                    2
                } else {
                    0
                }
            } else {
                if self.card_to_weight(c2, 2) > self.card_to_weight(c1, 1){
                    2
                } else {
                    1
                }
            }
        } else {
            if zero_one == 0 {
                if self.card_to_weight(c3, 3) > self.card_to_weight(c0, 0){
                    3
                } else {
                    0
                }
            } else {
                if self.card_to_weight(c3, 3) > self.card_to_weight(c1, 1){
                    3
                } else {
                    1
                }
            }
        }
    }

    pub fn intro_message(&mut self){
        println!("The first dealer will be player {}, so the first person that calls and plays will be player {}, enjoy your game!", (self.dealer ), (self.dealer + 1 ) % 4)
    }

    pub fn display_all_cards_in_hand(&self){
        for i in 1..5 {
            println!("User {}'s Cards: {}", (self.dealer + i) % 4, self.users[((self.dealer + i) % 4) as usize].cards);
        }
        println!();
    }

    pub fn display_playable(&self){
        for i in 1..5 {
            println!("User {}'s Playable Cards: {}", (self.dealer + i) % 4, self.users[((self.dealer + i) % 4) as usize].cards.filter_playable(self.get_wildsuit(), self.first_suit));
        }
    }

    pub fn playing_phase(&mut self){
        // println!("CARDS DEALT: {}", self.cards_dealt);
        for i in 0..self.cards_dealt {
            let mut cards_played = Vec::<Card>::with_capacity(4);
            println!("LENGTH {}", cards_played.len());
            for j in 1..5 {
                let player_number = (self.dealer + j ) as usize % 4;
                let mut playable = &self.users[player_number].cards.filter_playable(self.get_wildsuit(), self.first_suit);
                
                println!("Player {}'s playable cards are: \n{}", player_number, playable);
                for i in  1..(playable.get_length() + 1){
                    print!("{}  ", i);
                }
                println!("\nWhich card would you like to play? (Enter the number on the label below each card)");
                let card_number = user::get_card_number(i + 1);
                cards_played.push(playable.get(card_number - 1).expect("PLAYABLE GET ERROR"));
                println!();
            }
            cards_played.rotate_right(self.dealer as usize + 1);
            println!("CARDS PLAYED: {}", JokerDeck(cards_played));
        }
    }

    pub fn play_round(&mut self){
        self.update_round();
        println!("Dealing Cards...");
        self.deal_to_all_users();
        self.display_all_cards_in_hand();
        // self.display_playable();
        self.playing_phase();
    }

    pub fn play(&mut self){
        self.intro_message();
        self.play_round();
    }
}
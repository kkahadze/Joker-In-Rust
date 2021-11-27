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
use std::mem::take;
use std::thread::current;
use std::fmt;
pub struct JokerDeck(Vec<Card>);

impl JokerDeck {
    pub fn new() -> JokerDeck {
        let mut cards:Vec<Card> = Vec::with_capacity(36);
        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for rank in [Rank::Ace, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] {
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

    pub fn from_vec(cards: &Vec<Card>) -> JokerDeck{
        JokerDeck(cards.clone())
    }

    pub fn filter_playable(&self, wildsuit : Option<Suit>, first_suit : Option<Suit>) -> JokerDeck {
        match &first_suit {
            Some(first) => {
                let suit_copy = first.clone();
                let has_first_suit: bool = self.0.iter().fold(false, |sum: bool, val| sum || val.is_suit(suit_copy));
                
                if has_first_suit {
                    JokerDeck(self.0.clone().into_iter().filter(|card| card.is_joker() || card.is_suit(suit_copy)).collect())
                } else {
                    match wildsuit {
                        Some(wild   )   => {
                            let has_wild_suit: bool = self.0.iter().fold(false, |sum: bool, val| sum || val.is_suit(wild));
                            if has_wild_suit{
                                JokerDeck(self.0.clone().into_iter().filter(|card| card.is_joker() || card.is_suit(wild)).collect()) 
                            } else {
                                JokerDeck(self.0.clone())
                            }
                        },
                        None                 =>  JokerDeck(self.0.clone()),
                    }
                }
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

    pub fn remove(&mut self, card: &Card) {
        self.0.retain(|x| x != card);
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
    score:      i16,
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

        let mut out = true;
        if round  == 4 && play == 4 {
            out = false;
        } else if play == 8 && (round == 1 || round == 3){
            self.play = 1;
            self.round += 1;
            self.cards_dealt = self.play;
        } else if play == 4 && round == 2 {
            self.round = 3;
            self.play = 1;
            self.cards_dealt = self.play;
        } else if self.round == 0 && self.play == 0 {
            self.round = 1;
            self.play = 1;
            self.cards_dealt = self.play;
        } else {
            self.play += 1;
            self.cards_dealt = self.play;
        }
        // println!("CARDS DEALT: {}", self.cards_dealt);
        out
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
        self.set_calls(calls[0], calls[1], calls[2], calls[3]);
        Ok(())
    }

    pub fn deal_to_all_users(&mut self){
        for _ in 0..(self.cards_dealt){
            for i in 1..5 {
                self.users[((self.dealer + i) % 4) as usize].add_card(self.deck.pop().unwrap()); // unwrap failed
            }
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
        } else if taken == self.cards_dealt + 1 {
            println!("CALL {} TAKEN {} SCORE {} CARDS_DEALT {}", user.called, user.taken, user.score, self.cards_dealt);
            (self.cards_dealt * 100) as i16
        } else if taken != called {
            (taken * 10) as i16
        } else { 
            (taken * 50 + 50) as i16
        }
    }

    fn card_to_weight(&mut self, card: Card) -> u16{
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
        } else if self.first_suit.is_some() && card.suit == self.first_suit.unwrap() {
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

    pub fn compute_winner(&mut self, c0: Card, c1: Card, c2: Card, c3: Card, useless_joker: Option<u16>, second_joker: Option<u16>) -> u16 {
        let mut winner = 4; // This should never return
    
        let mut zero;
        let mut one;
        let mut two;
        let mut three;
        if second_joker.is_some(){
            match second_joker.unwrap(){
                0   => {
                    zero = self.card_to_weight(c0) * 2;
                    one = self.card_to_weight(c1);
                    two = self.card_to_weight(c2);
                    three = self.card_to_weight(c3);
                },
                1   => {
                    one = self.card_to_weight(c1) * 2;
                    zero = self.card_to_weight(c0);
                    two = self.card_to_weight(c2);
                    three = self.card_to_weight(c3);
                },
                2   => {
                    two = self.card_to_weight(c2) * 2;
                    zero = self.card_to_weight(c0);
                    one = self.card_to_weight(c1);
                    three = self.card_to_weight(c3);
                },
                3   => {
                    three = self.card_to_weight(c3) * 2;
                    zero = self.card_to_weight(c0);
                    one = self.card_to_weight(c1);
                    two = self.card_to_weight(c2);
                },
                _   => {
                    three = self.card_to_weight(c3);
                    zero = self.card_to_weight(c0);
                    one = self.card_to_weight(c1);
                    two = self.card_to_weight(c2);
                },
            } 
        } else {
            three = self.card_to_weight(c3);
            zero = self.card_to_weight(c0);
            one = self.card_to_weight(c1);
            two = self.card_to_weight(c2);
        }

        if useless_joker.is_some(){
            match useless_joker.unwrap() {
                0   => {
                    zero = 0;
                },
                1   => {
                    one = 0;
                },
                2   => {
                    two = 0;
                },
                3   => {
                    three = 0;
                }, 
                _   => (),
            }
        } 
        
        
        let zero_one;
        if zero > one {
            zero_one = 0;
        } else {
            zero_one = 1;
        }

        if two > three {
            if zero_one == 0 {
                if two > zero{
                    2
                } else {
                    0
                }
            } else {
                if two > one{
                    2
                } else {
                    1
                }
            }
        } else {
            if zero_one == 0 {
                if three > zero {
                    3
                } else {
                    0
                }
            } else {
                if three > one {
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

    pub fn playing_phase(&mut self) {
        // println!("CARDS DEALT: {}", self.cards_dealt);
        let mut starting_player = (self.dealer + 1) % 4;
        for _ in 0..self.cards_dealt {
            let mut cards_played = Vec::<Card>::with_capacity(4);
            // println!("LENGTH {}", cards_played.len());
            let mut useless_joker = None;

            for j in 0..4 {
                let player_number = (starting_player + j ) as usize % 4;
                
                let playable = &self.users[player_number].cards.filter_playable(self.get_wildsuit(), self.first_suit);
                
                println!("Player {}'s playable cards are: \n{}", player_number, playable);
                for z in  1..(playable.get_length() + 1){
                    print!("{}   ", z);
                }
                println!("\nWhich card would you like to play? (Enter the number on the label below each card)");
                let card_number = user::get_card_number(playable.get_length() as u16);
                let card = playable.get(card_number - 1).expect("PLAYABLE GET ERROR").copy();
                if j == 0 {
                    self.first_suit = Some(card.suit);
                } 

                let mut use_joker = None;
                let mut take_hand = None;
                let mut highs = None;
                if card.rank == Rank::Joker {
                    if j == 0{
                        take_hand = Some(user::get_bool_input("Would you like to take this hand?"));
                        highs = Some(user::get_bool_input("Would you like to call for the highs of a certain suit?"));
                        if highs.is_some() && highs.unwrap() {
                            let suit_of_highs = user::get_suit_input("Pick a suit by typing 'Spades', 'Diamonds', 'Hearts' or 'Clubs'");
                            self.first_suit = Some(suit_of_highs);
                        }

                        if take_hand.is_some() && !take_hand.unwrap(){
                            useless_joker = Some(player_number as u16);
                        }
                    } else {
                        use_joker = Some(user::get_bool_input("Would you like to use this Joker to take the hand?"));
                    }
                }
                self.users[player_number].cards.remove(&card);

                if (take_hand.is_some() && take_hand.unwrap()) || (use_joker.is_some() && use_joker.unwrap()){
                    cards_played.push(card);
                } else {
                    cards_played.push(Card::new(Rank::Six, Suit::Spades));
                }
                println!();
            }
            cards_played.rotate_right(self.dealer as usize + 1);
            println!("CARDS PLAYED: {}\nWILDCARD: {}", JokerDeck::from_vec(&cards_played), self.wildcard.expect("WILDCARD ERROR"));
            let winner;
            let mut first_joker = None;
            let mut second_joker = None;
            for j in 0..4{
                if cards_played[(starting_player + j) as usize% 4].rank == Rank::Joker{
                    if first_joker.is_none(){
                       first_joker = Some(j); 
                    } else {
                        second_joker = Some(j)
                    }
                }
            }

            winner = self.compute_winner(cards_played[0], cards_played[1], cards_played[2], cards_played[3], useless_joker, second_joker);
            
            println!("The Winner of this hand is player {}", winner);
            self.users[winner as usize].taken += 1;
            starting_player = winner;
            self.first_suit = None;
        }

        for i in 0..4 {
            self.users[i as usize].score += self.compute_score(&self.users[i as usize]);
        }
    }

    pub fn announce_results(&self){
        for i in 1..5 {
            let cur_user_num = (self.dealer + i) % 4;
            let cur_user = &self.users[cur_user_num as usize];
            println!("Player {} called {} and took {}, so they recieved {} points.", cur_user_num, cur_user.called, cur_user.taken, self.compute_score(&cur_user));
        }
    }

    pub fn announce_all_points(&self){
        for i in 1..5 {
            let cur_user_num = (self.dealer + i) % 4;
            let cur_user = &self.users[cur_user_num as usize];
            println!("Player {} has {} points.", cur_user_num, cur_user.score);
        }
    }

    pub fn play_round(&mut self){
        println!("Dealing Cards...");
        // println!("{}", self.cards_dealt);
        self.deal_to_all_users();
        self.display_all_cards_in_hand();
        self.ask_set_calls().expect("ERROR");
        // self.display_playable();
        self.playing_phase();
        self.announce_results();
        self.announce_all_points();
    }

    pub fn play(&mut self){
        self.intro_message();
        while self.update_round() {
            println!("\n");
            self.play_round();
        }
    }
}
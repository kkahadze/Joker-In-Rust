mod game;
mod card;
mod user;

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use crate::{card::{Card, Suit, Rank}, game::Game};

    use super::*;
    
    #[test]
    fn test_game_setup() {
        let game = game::Game::new();
        assert_eq!(0, game.cards_dealt);
        assert_eq!(0, game.users[0].id);
        assert_eq!(1, game.users[1].id);
        assert_eq!(2, game.users[2].id);
        
        assert!(game.dealer <= 3);
    }

    #[test]
    fn test_set_dealer() {
        let mut game = game::Game::new();
        let dealer = game.dealer;
        assert_eq!((dealer + 1) % 4, game::Game::next_dealer(&mut game))
    }

    #[test]
    fn test_update_round(){
        let mut game = game::Game::new();
        let mut correct = false;
        for _ in 0..8 {
            correct  = game.update_round() || correct;
        }
        assert_eq!(game.round, 1);
        assert_eq!(game.play, 8);
        assert!(correct);

        correct = game.update_round();
        assert_eq!(game.round, 2);
        assert_eq!(game.play, 1);
        assert!(correct);

        for _ in 0..4 {
            correct  = game.update_round() || correct;
        }
        assert_eq!(game.round, 3);
        assert_eq!(game.play, 1);
        assert!(correct);

        for _ in 0..8 {
            correct  = game.update_round() || correct;
        }
        assert_eq!(game.round, 4);
        assert_eq!(game.play, 1);
        assert!(correct);

        correct = true;

        for _ in 0..4 {
            correct  = game.update_round() && correct;
        }
        assert_eq!(game.round, 4);
        assert_eq!(game.play, 4);
        assert!(!correct);
    }

    #[test]
    fn test_compute_winner(){
        for i in 0..25{
            let mut game = game::Game::new();
            game.update_round();
            game.deal_to_all_users();
            let mut wild = Suit::Clubs;
            
            match wild{
                Suit::Clubs     => {
                    assert_eq!(game.compute_winner(
                        &Card::new(Rank::Ace, Suit::Clubs), 
                        &Card::new(Rank::Joker, Suit::Clubs), 
                        &Card::new(Rank::Ace, Suit::Hearts), 
                        &Card::new(Rank::Ace, Suit::Diamonds)
                    ), 1);
                },
                Suit::Diamonds     => {
                    assert_eq!(game.compute_winner(
                        &Card::new(Rank::Ace, Suit::Clubs), 
                        &Card::new(Rank::Six, Suit::Clubs), 
                        &Card::new(Rank::Ace, Suit::Hearts), 
                        &Card::new(Rank::Ace, Suit::Diamonds)
                    ), 3);
                },
                Suit::Hearts     => {
                    assert_eq!(game.compute_winner(
                        &Card::new(Rank::Ace, Suit::Clubs), 
                        &Card::new(Rank::Seven, Suit::Clubs), 
                        &Card::new(Rank::Ace, Suit::Hearts), 
                        &Card::new(Rank::Ace, Suit::Diamonds)
                    ), 2);
                },
                Suit::Spades     => {
                    assert_eq!(game.compute_winner(
                        &Card::new(Rank::Seven, Suit::Clubs), 
                        &Card::new(Rank::Eight, Suit::Spades), 
                        &Card::new(Rank::Nine, Suit::Hearts), 
                        &Card::new(Rank::Ten, Suit::Diamonds)
                    ), 1);
                    match game.dealer {
                        0   => {
                            assert_eq!(game.compute_winner(
                                &Card::new(Rank::Seven, Suit::Diamonds), 
                                &Card::new(Rank::Eight, Suit::Hearts), 
                                &Card::new(Rank::Nine, Suit::Diamonds), 
                                &Card::new(Rank::Ten, Suit::Diamonds)
                            ), 1);
                        },
                        1   => {
                            assert_eq!(game.compute_winner(
                                &Card::new(Rank::Seven, Suit::Diamonds), 
                                &Card::new(Rank::Eight, Suit::Hearts), 
                                &Card::new(Rank::King, Suit::Diamonds), 
                                &Card::new(Rank::Ten, Suit::Diamonds)
                            ), 2);
                        },
                        2   => {
                            assert_eq!(game.compute_winner(
                                &Card::new(Rank::Seven, Suit::Diamonds), 
                                &Card::new(Rank::Eight, Suit::Hearts), 
                                &Card::new(Rank::King, Suit::Diamonds), 
                                &Card::new(Rank::Ten, Suit::Hearts)
                            ), 3);
                        }
                        3   => {
                            assert_eq!(game.compute_winner(
                                &Card::new(Rank::Seven, Suit::Clubs), 
                                &Card::new(Rank::Eight, Suit::Hearts), 
                                &Card::new(Rank::King, Suit::Diamonds), 
                                &Card::new(Rank::Ten, Suit::Diamonds)
                            ), 0);
                        },
                        _ => (),
                    }
                },
            }
        }
    }

    #[test]
    fn test_play(){
        let mut game = Game::new();
        game.play();
    }
}
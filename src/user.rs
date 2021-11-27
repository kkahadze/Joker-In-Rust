use crate::card::Suit;
use std::io;

pub fn get_last_call(cards_dealt: u16, already_called: u16, user_number: u16) -> u16 {
    println!("How much does user {} call? ", user_number);

    let mut input_num = 0;
    let mut read_result;
    let mut read_valid;

    let mut input = String::new();
    read_result = io::stdin().read_line(&mut input);

    match read_result {
        Ok(_) => match input.trim().parse::<u16>() {
            Ok(value) => {
                if value as isize == cards_dealt as isize - already_called as isize {
                    read_valid = false;
                    println!(
                        "A number other than {} must be called. ",
                        cards_dealt - already_called
                    );
                } else {
                    read_valid = true;
                    input_num = value;
                }
            }
            Err(_) => read_valid = false,
        },
        Err(_) => {
            read_valid = false;
        }
    }

    while !read_valid {
        println!("Please enter a valid value: ");
        let mut input = String::new();
        read_result = io::stdin().read_line(&mut input);

        match read_result {
            Ok(_) => match input.trim().parse::<u16>() {
                Ok(value) => {
                    if value as isize == cards_dealt as isize - already_called as isize {
                        read_valid = false;
                        println!(
                            "A number other than {} must be called. ",
                            cards_dealt as isize - already_called as isize
                        );
                    } else {
                        read_valid = true;
                        input_num = value;
                    }
                }
                Err(_) => read_valid = false,
            },
            Err(_) => {
                read_valid = false;
            }
        }
    }
    input_num
}

pub fn get_call(user_number: u16) -> u16 {
    println!("How much does user {} call? ", user_number);

    let mut input_num = 0;
    let mut read_result;
    let mut read_valid;

    let mut input = String::new();
    read_result = io::stdin().read_line(&mut input);

    match read_result {
        Ok(_) => match input.trim().parse::<u16>() {
            Ok(value) => {
                read_valid = true;
                input_num = value
            }
            Err(_) => read_valid = false,
        },
        Err(_) => {
            read_valid = false;
        }
    }

    while !read_valid {
        println!("Please enter a valid value: ");
        let mut input = String::new();
        read_result = io::stdin().read_line(&mut input);

        match read_result {
            Ok(_) => match input.trim().parse::<u16>() {
                Ok(value) => {
                    read_valid = true;
                    input_num = value
                }
                Err(_) => read_valid = false,
            },
            Err(_) => {
                read_valid = false;
            }
        }
    }
    input_num
}

pub fn get_card_number(max: u16) -> u16 {
    let mut read_valid = false;
    let mut input_num = 10;
    let mut read_result;

    while !read_valid {
        if max == 1 {
            println!("Please enter 1 as it is the only card you may play: ");
        } else {
            println!("Please enter a valid value: 1 through {}", max);
        }
        let mut input = String::new();
        read_result = io::stdin().read_line(&mut input);

        match read_result {
            Ok(_) => match input.trim().parse::<u16>() {
                Ok(value) => {
                    if value > max || value < 1 {
                        read_valid = false;
                    } else {
                        read_valid = true;
                        input_num = value;
                    }
                }
                Err(_) => read_valid = false,
            },
            Err(_) => read_valid = false,
        }
    }
    input_num
}

pub fn get_bool_input(prompt: &str) -> bool {
    println!("{}", prompt);
    let mut input;
    let mut valid = false;
    let mut out: Option<bool> = None;
    while !valid {
        println!("Pick yes or no by typing 'y' or 'n'");
        // println!("LAST {}", input);
        input = String::from("");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some('\n') = input.chars().next_back() {
                    input.pop();
                }
                if let Some('\r') = input.chars().next_back() {
                    input.pop();
                }
                if input == "y" {
                    out = Some(true);
                    valid = true;
                } else if input == "n" {
                    out = Some(false);
                    valid = true;
                }
            }
            Err(_) => {
                println!("ERROR");
            }
        }
    }
    out.unwrap()
}

pub fn get_valid_suit(s1: Suit, s2: Suit, s3: Suit) -> Option<Suit> {
    let choice = get_bool_input("Would you like to choose a suit from these three cards? (You can choose to not have a wildsuit this round by saying no)");
    let mut suit = None;
    if choice {
        let mut valid = false;
        while !valid {
            suit = Some(get_suit_input("Choose a suit from the cards displayed: "));
            if suit.unwrap() == s1 || suit.unwrap() == s2 || suit.unwrap() == s3 {
                valid = true;
            }
        }
    }
    if choice {
        suit
    } else {
        None
    }
}

pub fn get_suit_input(prompt: &str) -> Suit {
    println!("{}", prompt);
    let mut input = String::new();
    let mut valid = false;
    let mut out: Option<Suit> = None;
    while !valid {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "Clubs" {
                    out = Some(Suit::Clubs);
                    valid = true;
                } else if input.trim() == "Spades" {
                    out = Some(Suit::Spades);
                    valid = true;
                } else if input.trim() == "Diamonds" {
                    out = Some(Suit::Diamonds);
                    valid = true;
                } else if input.trim() == "Hearts" {
                    out = Some(Suit::Hearts);
                    valid = true;
                }
            }
            Err(_) => {
                println!("ERROR");
            }
        }
    }
    out.unwrap()
}

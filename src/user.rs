use core::panic;
use std::{io, ptr::read, result};
use crate::card::Suit;

pub fn get_last_call(cards_dealt: u16, already_called : u16, user_number: u16) -> u16 {
    println!("How much does user {} call? ", user_number);

    let mut input_num = 0;
    let mut read_result;
    let mut read_valid = false;

    let mut input = String::new();
    read_result = io::stdin().read_line(&mut input);

    match read_result {
        Ok(_) => {
            match input.trim().parse::<u16>(){
                Ok(value) => {
                    if value as isize == cards_dealt as isize - already_called as isize{
                        read_valid = false;
                        println!("A number other than {} must be called. ", cards_dealt - already_called);
                    } else {
                        read_valid = true;
                        input_num = value;
                    }
                },
                Err(_) => read_valid = false,
            }
        }
        Err(_) => {
            read_valid = false;
        },
    }

    while !read_valid {
        println!("Please enter a valid value: ");
        let mut input = String::new();
        read_result = io::stdin().read_line(&mut input);

        match read_result {
            Ok(_) => {
                match input.trim().parse::<u16>(){
                    Ok(value) => {
                        if value as isize == cards_dealt as isize - already_called as isize{
                            read_valid = false;
                            println!("A number other than {} must be called. ", cards_dealt as isize - already_called as isize);
                        } else {
                            read_valid = true;
                            input_num = value;
                        }
                    },
                    Err(_) => read_valid = false,
                }
            }
            Err(_) => {
                read_valid = false;
            },
        }
    }
    input_num
}

pub fn get_call(user_number : u16) -> u16 {
    println!("How much does user {} call? ", user_number);

    let mut input_num = 0;
    let mut read_result;
    let mut read_valid = false;

    let mut input = String::new();
    read_result = io::stdin().read_line(&mut input);

    match read_result {
        Ok(_) => {
            match input.trim().parse::<u16>(){
                Ok(value) => {
                    read_valid = true;
                    input_num = value
                },
                Err(_) => read_valid = false,
            }
        }
        Err(_) => {
            read_valid = false;
        },
    }

    while !read_valid {
        println!("Please enter a valid value: ");
        let mut input = String::new();
        read_result = io::stdin().read_line(&mut input);

        match read_result {
            Ok(_) => {
                match input.trim().parse::<u16>(){
                    Ok(value) => {
                        read_valid = true;
                        input_num = value
                    },
                    Err(_) => read_valid = false,
                }
            }
            Err(_) => {
                read_valid = false;
            },
        }
    }
    input_num
}

pub fn get_card_number(max : u16) -> u16{
    let mut read_valid = false;
    let mut input_num = 10;
    let mut read_result;

    while !read_valid{
        if max == 1{
            println!("Please enter 1 as it is the only card you may play: ");
        } else {
            println!("Please enter a valid value: 1 through {}", max);
        }
        let mut input = String::new();
        read_result = io::stdin().read_line(&mut input);

        match read_result{
            Ok(_)      => {
                match input.trim().parse::<u16>(){
                    Ok(value)   => {
                        if value > max || value < 1 {
                            read_valid = false;
                        } else {
                            read_valid = true;
                            input_num = value;
                        }
                    },
                    Err(_) => read_valid = false,
                }
            },
            Err(_)    => read_valid = false,
        }
    }
    input_num
}


pub fn get_bool_input(prompt: &str) -> bool{
    println!("{}",prompt);
    let mut input = String::new();
    let mut valid = false;
    let mut out: Option<bool> = None;
    while !valid{
        println!("Pick yes or no by typing 'y' or 'n'");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "y" {
                    out = Some(true);
                    valid = true;
                } else if input.trim()== "n" {
                    out = Some(false);
                    valid = true;
                }
            },
            Err(_) => {
                println!("ERROR");
            },
        }
    }
    out.unwrap()
}

pub fn get_suit_input(prompt: &str) -> Suit{
    println!("{}",prompt);
    let mut input = String::new();
    let mut valid = false;
    let mut out: Option<Suit> = None;
    while !valid{
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().to_string() == "Clubs" {
                    out = Some(Suit::Clubs);
                    valid = true;
                } else if input.trim().to_string() == "Spades" {
                    out = Some(Suit::Spades);
                    valid = true;
                } else if input.trim().to_string() == "Diamonds" {
                    out = Some(Suit::Diamonds);
                    valid = true;
                } else if input.trim().to_string() == "Hearts" {
                    out = Some(Suit::Hearts);
                    valid = true;
                }
            },
            Err(_) => {
                println!("ERROR");
            },
        }
    }
    out.unwrap()
}
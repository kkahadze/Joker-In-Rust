use core::panic;
use std::{io, ptr::read, result};

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
                    if value == cards_dealt - already_called{
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
                        if value == cards_dealt - already_called{
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
        if (max == 1){
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

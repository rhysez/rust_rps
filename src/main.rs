use colored::*;
use rand::Rng;
use std::io;

fn main() {
    println!("{}", "WELCOME TO ROCK, PAPER, SCISSORS!".blue().bold());

    let mut round_counter: u32 = 1;

    // create an array of options that the computer can select
    let available_choices: [&str; 3] = ["rock", "paper", "scissors"];

    // create arrays of winning combinations
    let winning_combinations: [[&str; 2]; 3] = [
        ["rock", "scissors"],
        ["paper", "rock"],
        ["scissors", "paper"],
    ];

    // store the computer's choice by accessing a random value in available_choices
    loop {
        println!("ROUND {round_counter}! Please make a choice:");

        let computer_choice: &str =
            available_choices[rand::thread_rng().gen_range(0..=available_choices.len() - 1)];

        // create an empty string for storing the user's choice
        let mut user_choice: String = String::new();

        // store the user's choice via input
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        // using shadowing, convert user_choice to a &str, delete whitespace and /n
        let user_choice: &str = user_choice.as_str().trim();

        // store references to both choices in an array that we can compare against winning combinations
        let resulting_combination: [&str; 2] = [&user_choice, &computer_choice];

        println!("You picked: {}", resulting_combination[0]);
        println!("Computer picked: {}", resulting_combination[1]);

        // finally, we check if resulting_combination appears in the winning_combinations array
        if winning_combinations.contains(&resulting_combination) {
            println!("{}", "YOU WIN!".green().bold());
        } else if user_choice == computer_choice {
            println!(
                "{}",
                "It's a draw! Looks like you both picked the same."
                    .yellow()
                    .bold()
            );
        } else {
            println!("{}", "Oh no! You lose.".red().bold());
        }

        if round_counter == 3 {
            println!("{}", "GAME OVER".blue().bold());
            return;
        }

        round_counter += 1;
    }
}

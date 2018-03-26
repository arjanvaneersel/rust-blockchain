
#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    print!("difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("integer required");

    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);
     
    loop {
        println!("Menu");
        println!("1) New transaction");
        println!("2) Mine block");
        println!("3) Change difficulty");
        println!("4) Change reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                print!("enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                print!("enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());
                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            },
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated succesfully"),
                    false => println!("Block generation failed"),
                }
            },
            3 => {
                let mut new_diff = String::new();
                print!("enter difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed to update difficulty"),
                }

            },
            4 => {
                let mut new_reward = String::new();
                print!("enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed to update reward"),
                }
            },
            _ => {
                println!("\tinvalid option\t");
            }
        }
    }
}

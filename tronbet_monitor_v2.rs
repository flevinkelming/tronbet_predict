extern crate clap; // 2.32.0
// extern crate rand; // 0.5.5

use clap::{App, Arg};
// use rand::Rng;
use std::io;
use std::io::Write;
use std::process;
use std::collections::HashMap;

/*static ANSI_CLEAR_SCREEN: &'static str = &format!("{}[H{}[2J",
    27 as char, 27 as char);*/

fn main() {
    let matches = App::new("TRONbet Helper")
        .version("0.1")
        .author("Donate to <TWH95we5hehjusjEitUsfNcTAbBr14w66R> :)")
        .about("Useful metrics while playing, or simulate betting strategies!")
        .arg(Arg::with_name("trx")
            .help("Specify the initial TRX amount.")
            .short("t")
            .long("trx")
            .value_name("TRX")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("ante")
            .help("Specify the initial ANTE amount.")
            .short("a")
            .long("ante")
            .value_name("ANTE")
            .required(true)
            .takes_value(true)
        )
        .get_matches();
        
    let trx_amt = matches.value_of("trx").unwrap();
    // `trim()` necessary?
    let mut trx: f64 = match trx_amt.trim().parse() {
        Ok(n) => n,
        Err(_) => process::exit(1),
    };
    
    /*let ante_amt = matches.value_of("ante").unwrap();
    // `trim()` necessary?
    let mut ante: f64 = match ante_amt.trim().parse() {
        Ok(n) => n,
        Err(_) => process::exit(1),
    };*/
    
    let multiplier = 0.107; // over 10; 89% win chance
    let losing_number = 10.0;
    // let mining_stage_multiplier = 2.0; // 1-3
    // let trx_per_ante = 1_000.0 + 20.0 * mining_stage_multiplier;
    
    let mut reg_bets_won = 0.0;
    let mut total_reg_bets = 0.0;
    let mut recoup_bets_won = 0.0;
    let mut total_recoup_bets = 0.0;
    let mut streak: u32 = 0;
    let mut loss_streak: u32 = 0;

    let mut roll_frequency = HashMap::new(); // At most will contain 100 entries
    
    let mut current_roll = String::new();
    let mut prev_roll = String::new();
    
    let mut start = true;
    
    // ------------------------------------------------------------------------
    print!("{}[H{}[2J", 27 as char, 27 as char);
    io::stdout().flush().unwrap();
    
    loop {
        let mut number_rolled = String::new();
        
        print!("Enter the number rolled, or (q)uit: ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut number_rolled)
            .expect("Unable to read line.");
        
        if number_rolled.trim_end() == "q" {
            process::exit(0);
        }
        
        let roll: f64 = match number_rolled.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("There was an error parsing: {:?}", number_rolled);
                println!("Please make sure an integer (0-99) is entered.");
                continue
            },
        };
        
        let roll_entry = roll_frequency.entry(roll as u32).or_insert(0);
        *roll_entry += 1;
        
        if !start {
            prev_roll.pop();
        }
        
        match current_roll.pop() {
            Some(c) => prev_roll.push(c),
            _ => ()
        }
        
        if roll > losing_number {
            current_roll.push('w');
        } else {
            current_roll.push('l');
        }
        
        let recoup_bet = trx * 0.25;
        let reg_bet = recoup_bet * 0.10;
        
        // Explain strategy here...
        if start {
            if current_roll == "w".to_string() {
                trx += reg_bet * multiplier;
                reg_bets_won += 1.0;
                streak += 1;
            } else {
                trx -= reg_bet;
                streak = 0;
            }
            
            total_reg_bets += 1.0;
            start = false;
        } else if prev_roll == "l".to_string() {
            if current_roll == "w".to_string() {
                // 3 losses in a row
                if loss_streak > 1 {
                    trx += trx * multiplier;
                // 2 losses in a row
                } else if loss_streak > 0 {
                    trx += trx * 0.5 * multiplier;
                } else {
                    trx += recoup_bet * multiplier;   
                }
                
                recoup_bets_won += 1.0;
                streak += 1;
                loss_streak = 0;
            } else {
                // 3 losses in a row
                if loss_streak > 1 {
                    trx -= trx;
                // 2 losses in a row
                } else if loss_streak > 0 {
                    trx -= trx * 0.5;
                } else {
                    trx -= recoup_bet;   
                }
                
                streak = 0;
                loss_streak += 1;
            }
            
            total_recoup_bets += 1.0;
        } else {
            if current_roll == "w".to_string() {
                trx += reg_bet * multiplier;
                reg_bets_won += 1.0;
                streak += 1;
            } else {
                trx -= reg_bet;
                streak = 0;
            }
            
            total_reg_bets += 1.0;
        }
        
        let total_wins = reg_bets_won + recoup_bets_won;
        let total_bets = total_reg_bets + total_recoup_bets;
        let reg_ptg = fmt_f(reg_bets_won / total_reg_bets);
        let recoup_ptg = fmt_f(recoup_bets_won / total_recoup_bets);
        
        let mut median = 0.0;
        let mut mode: u32 = 0;
        let mut prev_freq = 0;
        for (roll, frequency) in &roll_frequency {
            median += (roll * frequency) as f64;
            
            if frequency > &prev_freq {
                mode = *roll;
                prev_freq = *frequency;
            }
        }
        
        let mut next_bet = 0.0;
        if current_roll == "w".to_string() {
            next_bet = trx * 0.25 * multiplier;
        } else if loss_streak > 1 {
            next_bet = trx;
        } else if loss_streak > 0 {
            next_bet = trx * 0.50;
        } else {
            next_bet = trx * 0.25;
        }
        
        print!("{}[H{}[2J", 27 as char, 27 as char);
        io::stdout().flush().unwrap();
        
        println!("Total {}/{}\nReg: {}/{} ({}%)\nRecoup: {}/{} ({}%)\n\n \
            Median: {}\nMode: {}\n\nStreak: {}\nNext Bet: {}",
            total_wins, total_bets, reg_bets_won, total_reg_bets, reg_ptg,
            recoup_bets_won, total_recoup_bets, recoup_ptg,
            median / (total_reg_bets + total_recoup_bets), mode, streak,
            next_bet,
        );
    }
}

fn fmt_f(f: f64) -> String {
    format!("{:.*}", 2, f)
}

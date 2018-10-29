use std::io;
use std::io::Write;

fn main() {
    let mut initial_trx = String::new();
    
    print!("{}[H{}[2J", 27 as char, 27 as char);
    print!("\n\n\n\t\tInitial TRX: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut initial_trx)
        .expect("Could not read line.");

    let mut trx: f64 = match initial_trx.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            panic!("There was a problem parsing input: {:?}", e)
        },
    };
    
    let multiplier = 0.107; // x1.107; 89% win chance
    let losing_number = 10.0;
    
    let mut reg_bets_hit = 0.0;
    let mut total_reg_bets = 0.0;
    let mut max_bets_hit = 0.0;
    let mut total_max_bets = 0.0;
    // let mut total_bets = 0.0;
    let mut total_amt_in_rolls = 0.0;
    let mut streak = 0;
    
    let mut current_roll = String::new();
    let mut prev_roll = String::new();
    
    let mut start = true;
    let mut double_loss = false;
    
    loop {
        let mut number_rolled = String::new();
        
        print!("\n\n\n\t\tNumber rolled, or (q)uit: ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut number_rolled)
            .expect("Could not read line.");
        
        if number_rolled == "q\n" {
            break
        }
            
        let roll: f64 = match number_rolled.trim().parse() {
            Ok(n) => n,
            Err(_) => break,
        };
        
        total_amt_in_rolls += roll;
        
        if !start {
            prev_roll.pop();
        }
        
        match current_roll.pop() {
            Some(c) => prev_roll.push(c),
            _ => (),
        }
        
        if roll > losing_number {
            current_roll.push('w');
        } else {
            current_roll.push('l');
        }
        
        let recoup_bet = trx * 0.25;
        let reg_bet = recoup_bet * 0.10;
        
        if start {
            if current_roll == "w".to_string() {
                trx += reg_bet * multiplier;
                reg_bets_hit += 1.0;
                 streak += 1;
            } else {
                trx -= reg_bet;
                streak = 0;
            }
            
            total_reg_bets += 1.0;
            start = false;
        } else if prev_roll == "l".to_string() {
            if current_roll == "w".to_string() {
                trx += recoup_bet * multiplier;
                max_bets_hit += 1.0;
                streak += 1;
            } else if double_loss {
                trx += (trx * 0.50) * multiplier;
                max_bets_hit += 1.0;
                double_loss = false;
                streak += 1;
            } else {
                trx -= recoup_bet;
                double_loss = true;
                streak = 0;
            }
            
            total_max_bets += 1.0;
        } else {
            if current_roll == "w".to_string() {
                trx += reg_bet * multiplier;
                reg_bets_hit += 1.0;
                streak += 1;
            } else {
                trx -= reg_bet;
                streak = 0;
            }
            
            total_reg_bets += 1.0;
        }
        
        // total_bets += 1.0;
        
        let total_num = reg_bets_hit + max_bets_hit;
        let total_denom = total_reg_bets + total_max_bets;
        let reg_ptg = fmt_f(reg_bets_hit / total_reg_bets);
        let max_ptg = fmt_f(max_bets_hit / total_max_bets);
        let avg = total_amt_in_rolls / (total_reg_bets + total_max_bets);
        
        let mut next_bet = 0.0;
        
        if current_roll == "w".to_string() {
            next_bet = trx * 0.25 * 0.107;
        } else if current_roll == "l".to_string() && double_loss {
            next_bet = trx * 0.50;
        } else {
            next_bet = trx * 0.25;
        }
        
        print!("{}[H{}[2J", 27 as char, 27 as char);
        io::stdout().flush().unwrap();
        
        println!("\n\n\n\t\tTotal: {}/{}\tReg: {}/{} ({}%)\tMax: {}/{} ({}%)\n \
            \t\tAverage: {}\tStreak: {}\tNext Bet: {}",
            total_num, total_denom, reg_bets_hit, total_reg_bets, reg_ptg,
            max_bets_hit, total_max_bets, max_ptg, avg, streak, next_bet
        );
    }
}

fn fmt_f(f: f64) -> String {
    format!("{:.*}", 2, f)
}

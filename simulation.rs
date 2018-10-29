extern crate rand; // 0.5.5

use rand::Rng;

fn main() {
    let mut _trx_bank = 0.0;
    let mut trx = 10_000.0;
    let mut bank_at = 25_000.0;
    let _ante_bank = 6_291.0;
    let mut ante = 0.0;
    
    let multiplier = 0.107; // x1.107; 89% win chance
    let losing_number = 10;
    let mining_stage = 2.0; // 1-3
    let trx_per_ante = 1_000.0 + (20.0 * mining_stage);
    let days = 10;
    
    let mut total_bets = 0.0;
    // let mut avg_roll = 0.0;
    let mut max_bets_hit = 0.0;
    let mut total_max_bets = 0.0;
    let mut reg_bets_hit = 0.0;
    let mut total_reg_bets = 0.0;
    
    let mut current_roll = String::new();
    let mut prev_roll = String::new();
    
    let mut start = true;
    let mut harbinger_of_losses = false;
    
    for d in 1..(days + 1) {
        while total_bets < (500.0 * d as f64) {
            if !start {
                prev_roll.pop();
            }
            
            match current_roll.pop() {
                Some(c) => prev_roll.push(c),
                _ => (),
            }
            
            let random_number = rand::thread_rng().gen_range(0, 100);
            
            if random_number > losing_number {
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
                } else {
                    trx -= reg_bet;
                }
                
                ante += reg_bet / trx_per_ante;
                total_reg_bets += 1.0;
                start = false;
            } else if prev_roll == "l" {
                if current_roll == "w".to_string() {
                    trx += recoup_bet * multiplier;
                    max_bets_hit += 1.0;
                    
                    ante += recoup_bet / trx_per_ante;
                } else if harbinger_of_losses {
                    trx += (trx * 0.50) * multiplier;
                    max_bets_hit += 1.0;
                    harbinger_of_losses = false;
                    
                    ante += (trx * 0.50) / trx_per_ante;
                } else {
                    trx -= recoup_bet;
                    harbinger_of_losses = true;
                }
                
                total_max_bets += 1.0;
            } else {
                if current_roll == "w".to_string() {
                    trx += reg_bet * multiplier;
                    reg_bets_hit += 1.0;
                } else {
                    trx -= reg_bet;
                }
                
                ante += reg_bet / trx_per_ante;
                total_reg_bets += 1.0;
            }
            
            total_bets += 1.0;
        }
        
        let usd_value = fmt_f(trx * 0.025);
        let max_ratio_ptg = fmt_f((max_bets_hit / total_max_bets) * 100.0);
        let reg_ratio_ptg = fmt_f((reg_bets_hit / total_reg_bets) * 100.0);
        
        println!("Day {}\tEnding TRX:\t{} (${})\n\tANTE Gained:\t{}\n \
            \tMax Ratio:\t{}/{} ({}%)\n\tReg. Ratio:\t{}/{} ({}%)\n",
            d, fmt_f(trx), usd_value, fmt_f(ante), max_bets_hit,
            total_max_bets, max_ratio_ptg, reg_bets_hit, total_reg_bets,
            reg_ratio_ptg,
            
        );
        
        // Bank half and raise the next threshold to twice the amount
        // of `bank_at`
        if trx > bank_at {
            trx /= 2.0;
            _trx_bank += trx * 2.0;
            bank_at *= 2.0;
        }
    }
    
    let dividends = fmt_f((ante + _ante_bank) * 9.5 * 0.023);
    println!("Total TRX:\t{}\nTotal ANTE:\t{} (${} per payout)\nUSD Value:\t${}",
        fmt_f(trx + _trx_bank), fmt_f(ante + _ante_bank), dividends,
        fmt_f((trx + _trx_bank + (ante + _ante_bank * 9.5)) * 0.023)
    );
}

fn fmt_f(f: f64) -> String {
    format!("{:.*}", 2, f)
}

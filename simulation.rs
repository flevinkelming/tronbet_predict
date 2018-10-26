extern crate rand; // 0.5.5

use rand::Rng;

fn main() {
    let trx_bank = 4_000.0;
    let mut trx = 10_000.00;
    let _ante_bank = 5_701.00;
    let mut ante_mined = 0.0;
    let basic = 10.0; // 100 seems to be the most profitable
    
    let multiplier = 1.037; // .037, .059, .071; 95% @ 1.037 is ideal
    let ante_stage = 1_000.0 + (10.0 * 2.0f64.powf(1.0));
    // let max_bets_per_day = 20;
    let bet_cycle = 5;
    
    let mut current = String::new();
    let mut previous = String::new();
    
    let mut total_bets = 0.0;
    let mut max_bets = 0.0;
    let mut total_max_bets = 0.0;
    let mut basic_bets = 0.0;
    let mut total_basic_bets = 0.0;
    
    let mut start = true;
    
    for s in 1..(bet_cycle + 1) {
        while total_bets < (500.0 * (s as f64)) {
            if !start {
                previous.pop();
            }
        
            match current.pop() {
                Some(c) => previous.push(c),
                _ => (),
            }
        
            let random_number = rand::thread_rng().gen_range(0, 100) as f64;
            
            if random_number > 4.0 {
                current.push('w');
            } else {
                current.push('l');
            }
            
            let half_trx = trx / 2.0;
            
            if start {
                if current == "w" {
                    trx += half_trx * multiplier - half_trx;
                } else {
                    trx -= half_trx;
                }
                
                ante_mined += half_trx / ante_stage;
                start = false;
            } else if previous == "l" {
                if current == "w" {
                    trx += half_trx * multiplier - half_trx;
                    max_bets += 1.0;
                } else {
                    trx -= half_trx;
                }
                
                total_max_bets += 1.0;
                ante_mined += half_trx / ante_stage;
                
            // Change `10` to variable for adjustment, e.g. to `1_000`
            } else if total_bets % 18.0 == 0.0 {
                if current == "w" {
                    trx += 10.0 * 24.0;
                    basic_bets += 1.0;
                } else {
                    trx -= 10.0;
                }
                
                total_basic_bets += 1.0;
                ante_mined += 10.0 / ante_stage;
            } else if !start {
                if current == "w" {
                    trx += basic * multiplier - basic;
                    basic_bets += 1.0;
                } else {
                    trx -= basic;
                }
                
                total_basic_bets += 1.0;
                ante_mined += basic / ante_stage;
            }

            total_bets += 1.0;
        }
        
        if trx < 0.0 {
            println!("You're wiped!");
            break
        }
        
        let usd_value = fmt_f(trx * 0.025);
        let max_ratio_ptg = fmt_f((max_bets / total_max_bets) * 100.0);
        let basic_ratio_ptg = fmt_f((basic_bets / total_basic_bets) * 100.0);
        
        println!("Day {}\tEnding TRX:\t{} (${})\n\tANTE Gained:\t{}\n \
            \tMax Ratio:\t{}/{} ({}%)\n\tBasic Ratio:\t{}/{} ({}%)\n",
            s, fmt_f(trx), usd_value, fmt_f(ante_mined), max_bets,
            total_max_bets, max_ratio_ptg, basic_bets, total_basic_bets,
            basic_ratio_ptg);
    }
    
    let dividends = fmt_f(ante_mined * 8.3 * 0.023);
    println!("Total TRX:\t{}\nTotal ANTE:\t{} (${} per payout)\nUSD Value:\t${}",
        fmt_f(trx+trx_bank), fmt_f(ante_mined), dividends,
        fmt_f((trx+trx_bank+ante_mined*8.5) * 0.025)
    );
    
}

fn fmt_f(f: f64) -> String {
    format!("{:.*}", 2, f)
}

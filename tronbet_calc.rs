// extern crate rand;

//use rand::Rng;

fn main() {
    // let secret_number = rand::thread_rng().gen_range(0, 100) as f64;

    let trx_banked = 0.0;
    let mut trx = 10_000.00; // 25_000 TRX is ideal start amount
    let total_ante = 5_701.334; // 43_245.9 TRX
    let mut ante_mined = 0.0;
    
    let multiplier = 1.048; // 95% @ 1.037 is ideal; 19/20 bets should win
    let ante_stage = 1_000.0 + (10.0 * 2.0f64.powf(2.0));
    let max_wager_per_day = 30;
    let wager_period = 1; // days
    let tb_max_bet = 135_500.0;
    
    // loop {}
    for w in 1..(max_wager_per_day * wager_period + 1) {
        // if loss % 20 bet 10 * 24 multiplier

        // TRONbet's max wager amount is around 136_000 TRX
        if trx < 272_000.0 {
            ante_mined += trx / ante_stage;
            trx += ((trx / 2.0) * multiplier) - (trx / 2.0);
        } /*else {
            ante_mined += tb_max_bet / ante_stage;
            trx += tb_max_bet * multiplier - tb_max_bet; 
        }*/
        
        if w % max_wager_per_day == 0 {
            trx -= 10.0; // With strategy should lose 100 TRX per max bet win
            let usd_value = fmt_f(trx * 0.023); // 1 TRX ~= $0.023
            let trx_value = fmt_f(ante_mined * 8.3); // 1 ANTE ~= 8.1 TRX
            
            println!("Day {}\tEnding TRX:\t{} (${})\n\tANTE Gained:\t{} ({} TRX)\n",
                w / max_wager_per_day, fmt_f(trx), usd_value, fmt_f(ante_mined),
                trx_value,
            );
        }
    }
    
    let dividends_payout = fmt_f(((total_ante+ante_mined) * 8.3) * 0.023);
    println!("Total TRX:\t{}\nTotal ANTE:\t{} (${} per payout)\nUSD Value:\t${}",
        fmt_f(trx_banked+trx), fmt_f(total_ante+ante_mined), dividends_payout,
        fmt_f((trx_banked+trx+(total_ante*8.3)+(ante_mined*8.3)) * 0.023),
    );
}

fn fmt_f(f: f64) -> String {
    format!("{:.*}", 2, f)
}

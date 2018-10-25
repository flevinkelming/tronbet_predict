fn main() {
    let mut trx = 17_391.0;
    let ante = 5_319.0;
    let mut ante_mined = 0.0;
    
    let multiplier = 1.037; // 95% @ 1.037 is ideal; 19/20 bets should win
    let ante_stage = 1_000.0 + (10.0 * 2.0f64.powf(1.0));
    let wager_per_day = 10;
    let wager_cycle = 5; // days
    
    for wager in 1..(wager_per_day*wager_cycle + 1) {
        if trx < 136_000.0 {
            ante_mined += trx / ante_stage;
            trx *= multiplier;
        } else {
            ante_mined += 135_000.0 / ante_stage;
            trx += 6_480.0; // max profit per bet @ 135_000 TRX wager
        }
        
        if wager%wager_per_day == 0 {
            trx -= 100.0; // With strategy should lose 100 TRX per max bet win
            let usd_value = fmt_f(trx * 0.023); // 1 TRX = 0.023 Dollars
            let trx_value = fmt_f(ante_mined * 8.1); // 1 ANTE = 8.1 TRX
            
            println!("Day {}\tEnding TRX:\t{} (${})\n \
                \tANTE Gained:\t{} ({} TRX)\n",
                wager / wager_per_day,
                fmt_f(trx),
                usd_value,
                fmt_f(ante_mined),
                trx_value,
            );
        }
    }
    
    let total_trx_value = (ante+ante_mined) * 8.1; // dividends
    let payout = fmt_f(total_trx_value * 0.023);
    println!("Total TRX dividends payout: {} (${})",
        fmt_f(total_trx_value),
        payout,
    );
}

fn fmt_f(f: f64) -> String {
    format!("{:.*}", 2, f)
}

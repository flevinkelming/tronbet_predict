use std::io;
use std::io::Write;

fn main() {
    let mut wins = 0.0;
    let mut losses = 0.0;
    let mut max_bet_wins = 0.0;
    let mut max_bet_losses = 0.0;

    let mut total_bets = 0.0;
    let mut total_wins_since_losses = 0.0;
    let mut streak = Vec::new();

    let mut start = true;

    loop {
        let mut wager = String::new();

        // ANSI Clear Screen
        print!("\n\n\n\t\tEnter wager result: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut wager)
            .expect("Could not read line.");

        if wager == "w\n" {
            streak.push('w');
            wins += 1.0;

            if !start && streak[(total_bets as usize)-1] == 'l' {
                max_bet_wins += 1.0;
            } else if start {
                // always start with a max bet
                max_bet_wins += 1.0;
                start = false;
            }

            total_bets += 1.0;
            total_wins_since_losses += 1.0;
        } else if wager == "l\n" {
            streak.push('l');
            losses += 1.0;

            if streak.len() > 1 && streak[(total_bets as usize)-1] == 'l' {
                max_bet_losses += 1.0;
            }

            total_bets += 1.0;
            total_wins_since_losses = 0.0;
        } else if wager == "q\n" {
            break
        }

        let win_ratio = format!("{}/{}", wins as u32, total_bets as u32);
        let win_ratio_ptg = format!("{:0.*}%", 2, (wins / total_bets) * 100.0);

        print!("{}[H{}[2J", 27 as char, 27 as char);
        println!("\n\n\t\tWin Ratio:\t\t\t\t{} ({})", win_ratio , win_ratio_ptg);
        println!("\t\tDaily max bets remaining:\t\t{}", (20.0-max_bet_wins) as u32);
        println!("\n\t\tLast loss was {} wagers ago.", total_wins_since_losses);
    }
}
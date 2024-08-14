pub struct Score {
    pub pc: i32,
    pub you: i32,
}

pub fn play(score: &mut Score) {
    println!("How many pieces do you want to play with?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut pieces: i32 = input.trim().parse().unwrap();

    println!("How many pieces can be removed per turn?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let pieces_per_turn: i32 = input.trim().parse().unwrap();

    let pc_starts: bool;
    let mut turn = 1;

    if pieces % (pieces_per_turn + 1) != 0 {
        pc_starts = true;
        println!("PC starts!");
    } else {
        pc_starts = false;
        println!("You start!");
    }

    let (mut pc_won, mut you_won) = (false, false);
    while pieces > 0 {
        if pc_starts {
            if turn % 2 != 0 {
                println!("PC's turn!");
                pc_play(&mut pieces, pieces_per_turn);
                println!("Remaining pieces: {}", pieces);
                if pieces == 0 {
                    pc_won = true;
                }
            } else {
                println!("Your turn!");
                you_play(&mut pieces, pieces_per_turn);
                println!("Remaining pieces: {}", pieces);
                if pieces == 0 {
                    you_won = true;
                }
            }
            turn += 1;
        } else {
            if turn % 2 != 0 {
                println!("Your turn!");
                you_play(&mut pieces, pieces_per_turn);
                println!("Remaining pieces: {}", pieces);
                if pieces == 0 {
                    you_won = true;
                }
            } else {
                println!("PC's turn!");
                pc_play(&mut pieces, pieces_per_turn);
                println!("Remaining pieces: {}", pieces);
                if pieces == 0 {
                    pc_won = true;
                }
            }
            turn += 1;
        }
    }

    if pc_won {
        println!("PC won!");
        score.pc += 1;
    } 

    if you_won {
        println!("You won!");
        score.you += 1;
    }
}

fn you_play(pieces: &mut i32, pieces_per_turn: i32) {
    loop {
        println!("How many pieces do you want to remove?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let play: i32 = input.trim().parse().unwrap();
        if play > 0 && play <= pieces_per_turn && play <= *pieces {
            *pieces -= play;
            println!("You removed {} pieces.", play);
            break;
        } else {
            println!("Invalid play! Try again.");
        }
    }
}

fn pc_play(pieces: &mut i32, pieces_per_turn: i32) {
    let (mut x, mut temp): (i32, i32) = (1, 0);
    while x <= pieces_per_turn {
        if (*pieces - x) % (pieces_per_turn + 1) == 0 {
            temp = x;
        }
        x += 1;
    }
    if temp == 0 && *pieces >= pieces_per_turn {
        *pieces -= pieces_per_turn;
        println!("PC removed {} pieces.", pieces_per_turn);
    } else if temp != 0 && temp <= pieces_per_turn {
        *pieces -= temp;
        println!("PC removed {} pieces.", temp);
    } else {
        *pieces -= *pieces;
        println!("PC removed {} pieces.", *pieces);
    }
}

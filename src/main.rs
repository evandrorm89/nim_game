use nim;

fn main() {
    println!("Welcome to the NIM game!");
    let mut option: u8;

    loop {
        println!("Choose:");
        println!("1. Isolated Match");
        println!("2. Championship");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        option = input.trim().parse().unwrap();
        if option == 1 || option == 2 {
            break;
        } else {
            println!("Invalid option! Try again.");
        }
    }

    let mut score = nim::Score { you: 0, pc: 0 };

    if option == 1 {
        println!("You chose Isolated Match!");
        nim::play(&mut score);
    } else {
        println!("You chose Championship!");

        for i in 1..4 {
            println!("*** Match {} ***", i);
            nim::play(&mut score);
        }
        println!("*** Championship ended! ***");
        println!("You: {} x PC: {}", score.you, score.pc);
    }
}

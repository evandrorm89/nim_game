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

    if option == 1 {
        println!("You chose Isolated Match!");
        nim::play();
    } else {
        println!("You chose Championship!");

        let you_points = 0;
        let pc_points = 0;

        for i in 1..4 {
            println!("*** Match {} ***", i);
            nim::play();
        }
        println!("*** Championship ended! ***");
        println!("You: {} x PC: {}", you_points, pc_points);
    }
}

use rand::RngExt;

fn main() {
    let random_number: i32 = rand::rng().random_range(1..=10);
    let mut number_of_guesses: i32 = 0;

    loop {
        println!("Guess the number 1 to 10:");

        let mut input = String::new();
        if std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input")
            == 0
        {
            break;
        }

        let guessed_number: i32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guessed_number == random_number {
            println!("You guessed the correct number: {random_number}, number of guesses: {number_of_guesses}");
            println!("You guessed the correct number: {guessed_number}");
            break;
        }

        number_of_guesses+=1;

        println!("Wrong guess, try again!");
    }
}

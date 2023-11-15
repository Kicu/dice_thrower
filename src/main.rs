use rand::Rng;
use std::io;

use dice_throwing::DiceThrowExpression;

// TODO
// add tests
// better error handling
// 

fn main() {
    print_welcome_text();

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error when reading line");

        let user_input = user_input.trim();

        if user_input == "exit" {
            break;
        }

        let result = handle_dice_throw(user_input);
        match result {
            Ok(res) => println!("Result for {}: {}", user_input, res),
            Err(err) => println!("Error: {}\nTry again", err),
        }
        println!("");
    }
}

fn handle_dice_throw(expr: &str) -> Result<i32, String> {
    let result = DiceThrowExpression::build(expr);
    match result {
        Ok(throw_expression) => {
            let die_number = throw_expression.die();

            let throws = throw_dice(die_number, throw_expression.dice_count());
            let throws_result = throws.iter().sum::<i32>();

            Ok(throws_result)
        }
        Err(err) => Err(err),
    }
}

fn throw_one_die(dice_num: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..(dice_num + 1))
}

fn throw_dice(dice_num: i32, dice_count: usize) -> Vec<i32> {
    println!("throwing d{} {} times", dice_num, dice_count);

    let mut result_vec = Vec::with_capacity(dice_count);

    for _ in 0..dice_count {
        let throw = throw_one_die(dice_num);
        result_vec.push(throw);
    }

    result_vec
}

fn print_welcome_text() {
    let separator_line = "****".repeat(8);
    println!("{}", separator_line);
    println!("**   Let's throw some dice!   **");
    println!("**   (type \"exit\" to finish)  **");
    println!("{}\n", separator_line);
}

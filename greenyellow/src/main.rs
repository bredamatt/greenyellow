use rand::prelude::*;
use std::io::Write;
use std::num::ParseIntError;

const NUM_DIGITS: usize = 4;
const RAN_MAX_RANGE: i32 = 9;
const RAN_MIN_RANGE: i32 = 1;

fn linear_search(item: &i32, arr: &[i32]) -> bool {
    let mut exists: bool = false;
    for value in arr.iter() {
        if item == value {
            exists = true;
            break // exit to avoid iterating over all guesses

        }
    }
    exists
}

fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    let mut result = String::new();
    for (i, guess_num) in guess.iter().enumerate() {
        if guess_num == &secret[i] {
            result.push_str("🟩 ");
        } else {
            match linear_search(&guess_num, &secret)  {
                true => result.push_str("🟨 "),
                false => result.push_str("⬜ "),
            }
        } 
    }
    result = result.trim().to_string();  // update in place?
    println!("Result: {:?}", result);
    result
}

fn gen_random(rng: &mut ThreadRng) -> i32 {
    let random: i32 = rng.gen_range(RAN_MIN_RANGE..RAN_MAX_RANGE);
    random
}

fn main() {
    let mut rng = rand::thread_rng();
    let secret = (0..NUM_DIGITS).map(|_| gen_random(&mut rng)).collect::<Vec<i32>>();
    let stdin = std::io::stdin();
    let mut buf = String::new();
    loop {
        buf.clear();
        print!("Guess: ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut buf).unwrap();
        let guess: Result<Vec<i32>, ParseIntError> = buf.trim().split_whitespace().map(|s| s.parse::<i32>()).collect();
        
        // TODO Bonus
        match guess {
            Ok(_) => {
                let result = calc_green_and_yellow(&guess.ok().unwrap(), &secret);
                if result == "🟩 🟩 🟩 🟩".to_string() {
                    println!("Congratulations! Correct guess!");
                    break;
                }
            },
            Err(error) => panic!("{:?}", error),
        }
    }
}

#[test]
fn test_green_and_yellow() {
    //                                    Guess         Secret
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]), "🟩 🟩 🟩 🟩".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]), "🟨 🟨 🟨 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]), "🟩 🟩 🟩 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]), "🟩 🟨 🟨 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]), "🟨 🟩 🟩 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]), "🟨 ⬜ ⬜ 🟨".to_string());
}

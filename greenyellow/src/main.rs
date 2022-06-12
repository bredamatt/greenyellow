use rand::prelude::*;
use std::io::Write;

const NUM_DIGITS: usize = 4;
const RAN_MAX_RANGE: i32 = 9;
const RAN_MIN_RANGE: i32 = 1;

// fn is_matching(guess: &[i32], secret: &[i32], result: &[String]) -> bool {
//     let mut matching: bool = false;
//     for (i, secret_num) in secret.iter().enumerate() {
//         println!("key={} value={}", i, number);
//         for (j, guess_num) in guess.iter().enumerate {
//             if secret_num == guess_num {
//                 matching = true;
//             } else {
//                 break
//             }
//         }
//     }
//     matching
// }

fn linear_search(item: &i32, arr: &[i32]) -> bool {
    let mut exists: bool = false;
    for value in arr.iter() {
        if item == value {
            exists = true;
            // exit as value was found once to avoid iterating over all guesses
            break
        }
    }
    exists
}

fn calc_green_and_yellow(secret: &[i32], guess: &[i32]) -> String {

    // Create result of type String
    let mut result = String::new();

    // Enumerate secret
    for (i, secret_num) in secret.iter().enumerate() {
        println!("key={} value={}", i, secret_num);
        // Check if secret at i is equal to guess at same index
        if secret_num == &guess[i] {
            result.push_str("🟩 ");
        } else {
            // If secret(i) != guess(i), see if secret(i) in guess
            match linear_search(&secret_num, &guess)  {
                true => result.push_str("🟨 "),
                false => result.push_str("⬜ "),
            }
        }
    }
    println!("{:?}", result);
    result
}

fn gen_random(rng: &mut ThreadRng) -> i32 {
    let random: i32 = rng.gen_range(RAN_MIN_RANGE..RAN_MAX_RANGE);
    random
}

fn main() {
    // create a new secret, generate random numbers between 1-9 using
    let mut rng = rand::thread_rng();
    let secret = (0..NUM_DIGITS)
        .map(|_| gen_random(&mut rng) )
        .collect::<Vec<i32>>();

    println!("{:?}", secret);


    let stdin = std::io::stdin();
    let mut buf = String::new();

    loop {
        // get input guess, numbers separated by space
        buf.clear();
        print!("guess: ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut buf).unwrap();
        let guess: Result<Vec<i32>, _> = buf.trim().split_whitespace().map(|s| s.parse()).collect();

        // TODO convert guess to &[i32]

        // TODO check guess with secret
        calc_green_and_yellow(&secret, &guess);
    }
}

#[test]
fn test_green_and_yellow() {
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]), "🟩 🟩 🟩 🟩".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]), "🟨 🟨 🟨 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]), "🟩 ⬜ ⬜ ⬜".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]), "🟨 🟩 🟩 🟨".to_string());
    assert_eq!(calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]), "🟨 ⬜ ⬜ 🟨".to_string());
}

use rand::prelude::*;
use std::io::Write;

const NUM_DIGITS: usize = 4;
const RAN_MAX_RANGE: i32 = 9;
const RAN_MIN_RANGE: i32 = 1;

fn is_matching(guess: &[i32], secret: &[i32]) -> bool {
    let mut matching: bool = false;
    for (index, number) in secret.iter().enumerate() {
        println!("key={} value={}", index, number);
    }
    matching
}

fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    /*
        1. Create result of type String
        2. Enumerate secret vector
        3. Compare each index in secret to value at guess Vector
        4. Iterate with two different functions, both taking a mutable borrowed slice
            4.1 First check if matching at same index, return Bool
            4.2 Then check if existing, return Bool
        5. If not existing (ie if False), push grey box to result String
    */

    let mut result = String::new();
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
        .collect::<Vec<_>>();

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

        // TODO check guess with secret
        let matching: bool = is_matching(&guess, &secret);
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

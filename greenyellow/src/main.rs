use rand::prelude::*;
use std::io::Write;

const NUM_DIGITS: usize = 4;
const RAN_MAX_RANGE: i32 = 9;
const RAN_MIN_RANGE: i32 = 1;

fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    let mut result = String::new();
    result
}

fn gen_random(rng: &mut ThreadRng) -> i32 {
    let random: i32 = rng.gen_range(RAN_MIN_RANGE..RAN_MAX_RANGE);
    random
}

fn main() {
    let mut rng = rand::thread_rng();
    // create a new secret, generate numbers between 1-9
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
//        let guess : Result<Vec<i32>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();
        let guess : Result<Vec<i32>, _> = buf.trim().split_whitespace().map(|s| s.parse()).collect();

        // TODO check guess with secret
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

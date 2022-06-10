use rand::prelude::*;
use std::io::Error;

fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    let matching = guess.iter().zip(secret.iter()).filter(|&(guess, secret)| guess == secret).count();
    let string = matching.to_string();
    string

}

fn generate_random_number(rand_num_generator: &mut ThreadRng) -> i32 {
    let rand = rand_num_generator.gen();
    rand
}

fn main() {
    let NUM_DIGITS = 3;
    let mut rng = thread_rng();

    let secret: Vec<i32> = (0..NUM_DIGITS).map(|_| generate_random_number(&mut rng)).collect();

    // let stdin = std::io::stdin();
    // let mut buf = String::new();
    
    // loop {
    //     buf.clear();
    //     print!("Guess: ");
    //     stdin.read_line(&mut buf).unwrap();
        
    //     // guess is separated by white spaces
    //     let guess: Result<Vec<i32>,Error> = buf.trim().split(' ').map(|s| s.parse()).collect();
    // }

    let guess: Vec<i32> = vec![1, 2, 3];

    let squares: String = calc_green_and_yellow(&guess, &secret);

    println!("Guess was: {:?}, secret was: {:?}, got {}", guess, secret,  squares);
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

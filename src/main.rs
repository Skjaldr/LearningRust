use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn prompt_user() {
	println!("Guess a number between 1 and 100");
}

fn get_input() -> u32 {
	
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Could not read line");
		let guess = guess.trim().parse().expect("Failed to get guess");
		guess
}

fn secret_number() -> u32 {
	
	let sec_num: u32 = rand::thread_rng().gen_range(1..=100);
	return sec_num;
}

fn win_condition(guess: &u32, secret_number: &u32) -> bool {

	
	match guess.cmp(&secret_number) {
		Ordering::Less => { 
			println!("too low!");
			return false
		},
		Ordering::Greater => { 
			println!("too high!");
			return false;
			
		}
		Ordering::Equal => {
			println!("You win!");
			return true;
		}
	}
		
}

fn game_loop() {
	let secret_number = secret_number();
	
	loop {
		prompt_user();
		let guess = get_input();
		if win_condition(&guess, &secret_number) == true {
			break;
		}
	}
	
}

fn main() {

	game_loop();
	
}
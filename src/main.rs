use std::io;
use std::process;
fn main() {
	let mut sum = String::new();

	println!("Welcome to my rust calculator!");
	println!("Please enter an initial value.");

	io::stdin().read_line(&mut sum).unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	let mut sum:f64 = sum.trim().parse().unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	loop {
		let mut choice = String::new();
		println!("\nNow select the operator you'd like to use on the number you entered by entering the number next to the operator or one of the operator symbols.");
		println!("1. Addition (+)");
		println!("2. Subtraction (-)");
		println!("3. Multiplication (*, x)");
		println!("4. Division (/, ÷)");
		println!("5. Remainder (%)");
		println!("6. Equals (=)");
		io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
			eprintln!("Application Error: {}", err);
			process::exit(1);
		});
		let choice = choice.trim() as &str;
		match choice {
			"1" | "1." | "+" => {
				println!("Enter the number you want to add to the current value of \"{}\".", sum);
				let choice = convert();
				println!("{} + {} = {}", sum, choice, sum + choice);
				sum += choice;
			}
			"2" | "2." | "-" => {
				println!("Enter the number you want to subtract to the current value of \"{}\".", sum);
				let choice = convert();
				println!("{} - {} = {}", sum, choice, sum - choice);
				sum -= choice;
			}
			"3" | "3." | "*" | "x" | "X" => {
				println!("Enter the number you want to multiply to the current value of \"{}\".", sum);
				let choice = convert();
				println!("{} * {} = {}", sum, choice, sum * choice);
				sum *= choice;
			}
			"4" | "4." | "/" | "÷" => {
				println!("Enter the number you want to divide to the current value of \"{}\".", sum);
				let choice = convert();
				println!("{} / {} = {}", sum, choice, sum / choice);
				sum /= choice;
			}
			"5" | "5." | "%" => {
				println!("Enter the number you want to see the remainder for to the current value of \"{}\".", sum);
				let choice = convert();
				println!("{} % {} = {}", sum, choice, sum % choice);
				sum %= choice;
			}
			"6" | "6." | "=" => {
				println!("Your final value is \"{}\".", sum);
				process::exit(0);
			}
			&_ => {
				eprintln!("You entered an invalid value.");
				process::exit(1);
			}
		}
	}
}
fn convert()->f64 {
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	let choice:f64 = choice.trim().parse().unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
				});
	return choice;
}
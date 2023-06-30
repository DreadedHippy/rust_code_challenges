fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
		let mut duration = String::new();
		
		std::io::stdin()
		.read_line(&mut duration)
		.unwrap();

		let mut duration: i32 = duration.trim().parse().unwrap();

		println!("{}", if duration > 30 {"YES"} else {"NO"})

	}

}
fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
		let mut battery_percentage = String::new();
		
		std::io::stdin()
		.read_line(&mut battery_percentage)
		.unwrap();


		let battery_percentage: i32 = battery_percentage.trim().parse().unwrap();

		println!("{}", if battery_percentage < 80 {"NO"} else {"YES"})

	}

}
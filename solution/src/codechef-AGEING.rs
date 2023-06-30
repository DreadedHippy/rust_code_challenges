fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
		let mut chef_age = String::new();
		
		std::io::stdin()
		.read_line(&mut chef_age)
		.unwrap();

		let chef_age = chef_age.trim().parse::<i32>().unwrap();

		println!("{}", chef_age - 10)

	}

}
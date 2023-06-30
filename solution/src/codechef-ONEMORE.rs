fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
		let mut minutes_to_exam = String::new();
		
		std::io::stdin()
		.read_line(&mut minutes_to_exam)
		.unwrap();

		let minutes_to_exam: i32 = minutes_to_exam.trim().parse().unwrap();

		println!("{}", if minutes_to_exam > 24 {"YES"} else {"NO"})

	}

}
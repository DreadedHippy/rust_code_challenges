fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
		let mut problem = String::new();
		
		std::io::stdin()
		.read_line(&mut problem)
		.unwrap();

		let mut iterable = problem.trim().split_whitespace();

		let population = iterable.next().unwrap().parse::<i32>().unwrap();
		let capacity = iterable.next().unwrap().parse::<i32>().unwrap();

		println!("{}", if population > capacity {"NO"} else {"YES"})

	}

}
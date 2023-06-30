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

        let mut number_of_candies: i32 = problem.trim().parse().unwrap();

        println!("{}", if number_of_candies % 3 == 0 {"YES"} else {"NO"})

	}

}
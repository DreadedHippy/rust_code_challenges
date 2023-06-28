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

        let problem: i32 = problem.trim().parse().unwrap();

        println!("{}", if problem < 67 || problem > 45000 {"NO"} else {"YES"})

	}

}

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

        let top_ten_prize: i32 = iterable.next().unwrap().parse().unwrap();
        let bottom_ninety_prize: i32 = iterable.next().unwrap().parse().unwrap();

        println!("{}", (top_ten_prize * 10) + (bottom_ninety_prize * 90))

	}

}
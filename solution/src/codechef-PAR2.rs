fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
        let mut chocolates = String::new();
        
        std::io::stdin()
        .read_line(&mut chocolates)
        .unwrap();

        let mut chocolates: i32 = chocolates.trim().parse().unwrap();

        println!("{}", if chocolates % 2 == 0 {"YES"} else {"NO"})

	}

}
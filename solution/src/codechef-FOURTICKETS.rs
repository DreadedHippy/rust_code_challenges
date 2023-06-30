fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
        let mut cost_of_ticket = String::new();
        
        std::io::stdin()
        .read_line(&mut cost_of_ticket)
        .unwrap();

        let cost_of_ticket: i32 = cost_of_ticket.trim().parse().unwrap();

        println!("{}", if cost_of_ticket*4 > 1000 {"NO"} else {"YES"})

	}

}
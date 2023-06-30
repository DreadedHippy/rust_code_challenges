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

        let first_num = iterable.next().unwrap().parse::<i32>().unwrap();
        let second_num = iterable.next().unwrap().parse::<i32>().unwrap();
        let third_num = iterable.next().unwrap().parse::<i32>().unwrap();

        let mut vector = vec![first_num, second_num, third_num];

        vector.sort();

        println!("{}", vector[1])

	}

}
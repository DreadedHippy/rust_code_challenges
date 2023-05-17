// Problem
// The working hours of Chef’s kitchen are from X pm to Y pm (1≤X<Y≤12).

// Find the number of hours Chef works.

// Input Format
// The first line of input will contain a single integer T, denoting the number of test cases.
// Each test case consists of two space-separated integers X and Y — the starting and ending time of working hours respectively.

// Output Format
// For each test case, output on a new line, the number of hours Chef works.

// Constraints
// 1≤T≤100
// 1≤X<Y≤12
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

        let x: i32 = iterable.next().unwrap().parse().unwrap();
        let y: i32 = iterable.next().unwrap().parse().unwrap();

        println!("{}", y - x)


	}

}
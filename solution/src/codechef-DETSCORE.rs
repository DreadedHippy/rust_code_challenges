// Problem
// Chef appeared for a placement test.

// There is a problem worth X points.
// Chef finds out that the problem has exactly 10 test cases.
// It is known that each test case is worth the same number of points.

// Chef passes N test cases among them. Determine the score Chef will get.

// NOTE: See sample explanation for more clarity.

// Input Format
// First line will contain T, number of test cases. Then the test cases follow.
// Each test case contains of a single line of input, two integers N, the total points for the problem and the number of test cases which pass for Chef's solution.

// Output Format
// For each test case, output the points scored by Chef.

// Constraints
// 1≤T≤100
// 10≤X≤200
// 0≤N≤10
// X is a multiple of 10.

fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(&mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
			// ! There are 10! test cases per question
			// ! Each test case is worth the same number of points;
			// ! The first integer is the total number of points; 

			let mut problem = String::new();
			
			std::io::stdin()
			.read_line(&mut problem)
			.unwrap();

			let mut iter = problem.trim().split_whitespace();

			
			let number_of_test_cases = 10;
			let total_points: i32 = iter.next().unwrap().parse().unwrap();
			let test_cases_passed: i32 = iter.next().unwrap().parse().unwrap();

			println!("{}", total_points / number_of_test_cases * test_cases_passed )

	}

}
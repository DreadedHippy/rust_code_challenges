// Problem
// DAIICT college students want to attend an IPL match.

// A total of N students from the college want to go while only M tickets are available for the match.
// Determine how many students won't be able to book tickets.

// Input Format
// The first line of input will contain a single integer T, denoting the number of test cases.
// Each test case consists of two space-separated integers N and M — the number of students wants to go and the total number of tickets available, respectively.

// Output Format
// For each test case, output on a new line the number of students who won't be able to book tickets.

// Constraints
// 1≤T≤1000
// 1≤N,M≤10^5
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

        let mut iter = problem.trim().split_whitespace();

        let n: i32 = iter.next().unwrap().parse().unwrap();
        let m: i32 = iter.next().unwrap().parse().unwrap();

        println!("{}", if m >= n {0} else {n - m})



	}

}
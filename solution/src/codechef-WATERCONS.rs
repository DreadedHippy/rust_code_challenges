// Problem

// Recently, Chef visited his doctor. The doctor advised Chef to drink at least 2000 ml of water each day.

// Chef drank X ml of water today. Determine if Chef followed the doctor's advice or not.

// Input Format
// The first line contains a single integer T — the number of test cases. Then the test cases follow.
// The first and only line of each test case contains one integer X — the amount of water Chef drank today.

// Output Format
// For each test case, output YES if Chef followed the doctor's advice of drinking at least 2000 ml of water. Otherwise, output NO.
// You may print each character of the string in uppercase or lowercase (for example, the strings YES, yEs, yes, and yeS will all be treated as identical).

// Constraints
// 1≤T≤2000
// 1≤X≤4000

fn main() {
	let mut number_of_test_cases: String = String::new();
	std::io::stdin()
	.read_line(& mut number_of_test_cases)
	.unwrap();

	let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

	for _test_case in 1..=number_of_test_cases {
			let mut water_consumed = String::new();

			std::io::stdin().read_line(&mut water_consumed).unwrap();        
			let water_consumed: usize = water_consumed.trim().parse().unwrap();
			println!("{}", if water_consumed >= 2000 {"YES"} else {"NO"})
	}

}
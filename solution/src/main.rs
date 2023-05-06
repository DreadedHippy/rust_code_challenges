// Problem

// Chef has been working hard to compete in MasterChef.
// He is ranked X out of all contestants.
// However, only 10 contestants would be selected for the finals.

// Check whether Chef made it to the top 10 or not?

// Input Format
// The first line of input will contain a single integer T, denoting the number of test cases.
// Each test case consists of one integers X — the current rank of Chef.

// Output Format
// For each test case, output on a new line, YES, if Chef made it to the top 10 and NO otherwise.

// Each character of the output may be printed in either uppercase or lowercase. That is, the strings NO, no, nO, and No will be treated as equivalent.

// Constraints
// 1≤T≤100
// 1≤X≤100

fn main() {
    let mut number_of_test_cases: String = String::new();
    std::io::stdin()
    .read_line(&mut number_of_test_cases)
    .unwrap();

    let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

    for _test_case in 1..=number_of_test_cases {
        let mut position = String::new();
        
        std::io::stdin()
        .read_line(& mut position)
        .unwrap();

        let position: usize = position.trim().parse().unwrap();

        println!("{}", if position > 10 {"NO"} else {"YES"})
    }
}
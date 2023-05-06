// Problem
// Chef wants to become fit for which he decided to walk to the office and return home by walking.
// It is known that Chef's office is X km away from his home.

// If his office is open on 5 days in a week, find the number of kilometers Chef travels through office trips in a week.

// Input Format
// First line will contain T, number of test cases. Then the test cases follow.
// Each test case contains of a single line consisting of single integer X.

// Output Format
// For each test case, output the number of kilometers Chef travels through office trips in a week.

// Constraints
// 1≤T≤10
// 1≤X≤10

fn main() {
    let mut number_of_test_cases: String = String::new();
    std::io::stdin()
    .read_line(&mut number_of_test_cases)
    .unwrap();

    let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

    for _test_case in 1..=number_of_test_cases {
        let mut distance = String::new();
        
        std::io::stdin()
        .read_line(&mut distance)
        .unwrap();

        let distance: i32 = distance.trim().parse().unwrap();

        println!("{}", distance * 2 * 5)

    }

}
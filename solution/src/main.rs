// Problem
// Chef is fond of burgers and decided to make as many burgers as possible.

// Chef has A patties and B buns.
// To make 1 burger, Chef needs 1 patty and 1 bun.
// Find the maximum number of burgers that Chef can make.

// Input Format
// The first line of input will contain an integer T — the number of test cases.
// The description of T test cases follows.
// The first and only line of each test case contains two space-separated integers A and B, the number of patties and buns respectively.

// Output Format
// For each test case, output the maximum number of burgers that Chef can make.

// Constraints
// 1≤T≤1000
// 1≤A,B≤10^5

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

        let a: i32 = iterable.next().unwrap().parse().unwrap();
        let b: i32 = iterable.next().unwrap().parse().unwrap();

        // we need to return the smaller number between a and b
        println!("{}", if a < b {a} else {b})



    }

}
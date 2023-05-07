// Problem
// CodeChef recently revamped its practice page to make it easier for users to identify the next problems they should solve by introducing some new features:

// Recent Contest Problems - contains only problems from the last 2 contests
// Separate Un-Attempted, Attempted, and All tabs
// Problem Difficulty Rating - the Recommended dropdown menu has various difficulty ranges so that you can attempt the problems most suited to your experience
// Popular Topics and Tags
// Our Chef is currently practicing on CodeChef and is a beginner. The count of ‘All Problems’ in the Beginner section is X.
// Our Chef has already ‘Attempted’ Y problems among them. How many problems are yet ‘Un-attempted’?

// Input Format
// The first and only line of input contains two space-separated integers X and Y — the count of 'All problems' in the Beginner's section and the count of Chef's 'Attempted' problems, respectively.

// Output Format
// Output a single integer in a single line — the number of problems that are yet 'Un-attempted'

// Constraints1≤Y≤X≤1000

// Subtasks
// Subtask 1 (100 points):
// - Original constraints.

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
    
    let mut problem = String::new();        
        
    std::io::stdin()
    .read_line(&mut problem)
    .unwrap();

    let mut iter = problem.trim().split_whitespace();

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



    println!("{}", x - y);

}
// Problem
// According to a recent survey, Biryani is the most ordered food. Chef wants to learn how to make world-class Biryani from a MasterChef.
// Chef will be required to attend the MasterChef's classes for X weeks, and the cost of classes per week is Y coins.
// What is the total amount of money that Chef will have to pay?

// Input Format
// The first line of input will contain an integer T — the number of test cases.
// The description of T test cases follows.

// The first and only line of each test case contains two space-separated integers X and Y, as described in the problem statement.

// Output Format
// For each test case, output on a new line the total amount of money that Chef will have to pay.

// Constraints
// 1≤T≤10^4
// 1≤X,Y≤100

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
        let x: i32 = iter.next().unwrap().trim().parse().unwrap();
        let y: i32 = iter.next().unwrap().trim().parse().unwrap();

        println!("{}", x * y)

    }

}
// Problem
// In Chefland, everyone who earns strictly more than Y rupees per year, has to pay a tax to Chef.
// Chef has allowed a special scheme where you can invest any amount of money and claim exemption for it.

// You have earned (X>Y) rupees this year. Find the minimum amount of money you have to invest so that you don't have to pay taxes this year.

// Input Format

// The first line of input will contain a single integer T, denoting the number of test cases.
// Each test case consists of a single line of input consisting of two space separated integers X and Y denoting the amount you earned and the amount above which you will have to pay taxes.

// Output Format
// For each test case, output a single integer, denoting the minimum amount you need to invest.

// Constraints
// 1≤T≤100
// 1≤Y<X≤100

use std::io::Read;

fn main() {
    let mut number_of_test_cases = String::new();
    std::io::stdin().read_line(&mut number_of_test_cases).unwrap();

    let number_of_test_cases: u32 = number_of_test_cases.trim().parse().unwrap();

    for test_case in 1..=number_of_test_cases {
        let mut input = String::new();

        std::io::stdin()
        .read_line(&mut input)
        .unwrap();
        let mut iterable = input.trim().split_whitespace();
        let earned: i32 = iterable.next().unwrap().parse().unwrap();
        let taxable: i32 = iterable.next().unwrap().parse().unwrap();
        println!("{}", earned - taxable)

    }
}

//?Boilerplate code

// std::io::stdin()
// .read_line(&mut x)
// .unwrap();
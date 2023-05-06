// Problem
// Chef is playing Ludo. According to the rules of Ludo, a player can enter a new token into the play only when he rolls a 6 on the die.

// In the current turn, Chef rolled the number  X on the die.
// Determine if Chef can enter a new token into the play in the current turn or not.

// Input Format
// The first line contains a single integer T — the number of test cases.
// Then the test cases follow.
// The first and only line of each test case contains one integer X — the number rolled by the Chef on the die.

// Output Format
// For each test case, output YES if the Chef can enter a new token in the game. Otherwise, output NO.

// You may print each character of YES and NO in uppercase or lowercase (for example, yes, yEs, Yes will be considered identical).

fn main() {
    let mut number_of_test_cases: String = String::new();
    std::io::stdin()
    .read_line(&mut number_of_test_cases)
    .unwrap();

    let number_of_test_cases: i32 = number_of_test_cases.trim().parse().unwrap();

    for _test_case in 1..=number_of_test_cases {
        let mut result = String::new();
        
        std::io::stdin()
        .read_line(&mut result)
        .unwrap();
        
        let result: i32 = result.trim().parse().unwrap();

        println!("{}", if result == 6 {"YES"} else {"NO"})

    }

}
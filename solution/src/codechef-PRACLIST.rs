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

fn main() {
    
	let mut problem = String::new();        
			
	std::io::stdin()
	.read_line(&mut problem)
	.unwrap();

	let mut iter = problem.trim().split_whitespace();

	let x: i32 = iter.next().unwrap().parse().unwrap();
	let y: i32 = iter.next().unwrap().parse().unwrap();

	println!("{}", x - y);

}
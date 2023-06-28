fn main() {
	let mut problem = String::new();

	std::io::stdin()
	.read_line(&mut problem)
	.unwrap();

	let mut iterable = problem.trim().split_whitespace();

	let a: i32 = iterable.next().unwrap().parse().unwrap();
	let b: i32 = iterable.next().unwrap().parse().unwrap();
	let c: i32 = iterable.next().unwrap().parse().unwrap();

	println!("{}", (a - b), (a-(b+c)))
}

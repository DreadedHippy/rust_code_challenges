fn main() {
	let mut problem = String::new();

	std::io::stdin()
	.read_line(&mut problem)
	.unwrap();

	let mut iterable = problem.trim().split_whitespace();
	
	let threshold: i32 = iterable.next().unwrap().parse().unwrap();
	let current_speed: i32 = iterable.next().unwrap().parse().unwrap();


	println!("{}", if current_speed <= threshold {"NO"} else {"YES"})

}

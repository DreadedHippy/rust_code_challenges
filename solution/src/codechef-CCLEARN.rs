fn main() {
	let mut courses = String::new();

	std::io::stdin()
	.read_line(&mut courses)
	.unwrap();

	let courses:i32 = courses.trim().parse().unwrap();

	println!("{}", courses * 2)

}

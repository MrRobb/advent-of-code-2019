mod day1;
mod day2;
fn main() {
	let mains = [day1::main, day2::main];

	for (day, main) in mains.iter().enumerate() {
		println!(
			"------------------------------------ DAY {} ------------------------------------",
			day
		);
		main();
		println!();
	}
}

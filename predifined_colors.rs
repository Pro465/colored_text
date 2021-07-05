use colored::*;

fn main() {
	let color = Color {
		foreground: colors::BLACK,
		background: colors::WHITE
	};
	
	color.set();
	
	println!("Hello, World!");
}

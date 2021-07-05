pub type ColorType = (u8,u8,u8);

#[derive(Clone, Copy, Debug)]
pub struct Color {
	pub foreground: ColorType,
	pub background: ColorType
}

impl Color {
	pub fn set(&self) {
		use std::io::{ Write , stdout };
		let mut out = stdout();

		out.write_all(

		&(0x1b..=0x1b)
			.chain(format!(
			"[38;2;{};{};{}m",
			self.foreground.0,
			self.foreground.1,
			self.foreground.2				
			).bytes())
			
			.chain(0x1b..=0x1b)
			
			.chain(format!(
			"[48;2;{};{};{}m",
			self.background.0,
			self.background.1,
			self.background.2,
			).bytes())
			.collect::<Vec<u8>>()

		)
		   .expect("could not set color");
		
		out.flush()
		   .unwrap();
	}
}

pub mod colors;

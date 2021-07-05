use colored::*;

fn main() {
        let color = Color {
                foreground: (255,0,255),
                background: (0,128,0)
        };

        color.set();

        println!("Hello, World!");
}

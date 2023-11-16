extern crate colored;

use colored::{Color, Colorize};

pub fn new_line() {
	println!("\n");
}

pub fn line(text: &str) {
	println!("{}", text);
}

pub fn yellow(text: &str) {
	println!("{}", text.color(Color::Yellow));
}

pub fn red(text: &str) {
	println!("{}", text.color(Color::Red));
}

pub fn blue(text: &str) {
	println!("{}", text.color(Color::Blue));
}

pub fn green(text: &str) {
	println!("{}", text.color(Color::Green));
}

pub fn cyan(text: &str) {
	println!("{}", text.color(Color::Cyan));
}

pub fn magenta(text: &str) {
	println!("{}", text.color(Color::Magenta));
}

pub fn yellow_header(text: &str) {
	color_header(text, Color::Yellow)
}

pub fn red_header(text: &str) {
	color_header(text, Color::Red)
}

pub fn blue_header(text: &str) {
	color_header(text, Color::Blue)
}

pub fn green_header(text: &str) {
	color_header(text, Color::Green)
}

pub fn cyan_header(text: &str) {
	color_header(text, Color::Cyan)
}

pub fn magenta_header(text: &str) {
	color_header(text, Color::Magenta)
}

fn color_header(text: &str, color: Color) {
	new_line();
	println!("{}", text.color(color));
	println!("{}", "-----------------------------------------".color(color));
}

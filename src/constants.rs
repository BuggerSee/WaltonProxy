extern crate ansi_term;
extern crate byteorder;

use self::byteorder::BigEndian;
use self::byteorder::ReadBytesExt;
use constants::ansi_term::Colour::*;

pub static STANDARD_COLOR: &'static str = "yellow";
pub static SUCCESS_COLOR: &'static str = "green";
pub static FAIL_COLOR: &'static str = "red";
pub static WALTON_DATA_COLOR: &'static str = "cyan";
pub static MING_DATA_COLOR: &'static str = "purple";
pub static AMOUNT_GPU: &'static i32 = &1;
pub static PORT_NUMBER_VECTOR: &'static [&'static str;7] = &["12140","12126","12127","12128","12129","12130","12131"];

pub fn print_color(input: &str, color: &String) {
    if color.contains("red") {
        println!("{}", Red.paint(input));
    } else if color.contains("blue") {
        println!("{}", Blue.paint(input));
    } else if color.contains("green") {
        println!("{}", Green.paint(input));
    } else if color.contains("yellow") {
        println!("{}", Yellow.paint(input));
    } else if color.contains("purple") {
        println!("{}", Purple.paint(input));
    } else if color.contains("cyan") {
        println!("{}", Cyan.paint(input));
    }
}

pub fn print_block(mut slice: &[u8]) {
    let num = slice.read_u32::<BigEndian>().unwrap();
    print_color(&format!("  BlockNumber: {}", num), &SUCCESS_COLOR.to_owned());
}
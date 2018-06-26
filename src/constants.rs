extern crate ansi_term;
extern crate byteorder;
extern crate rand;

use self::rand::Rng;
use self::byteorder::BigEndian;
use self::byteorder::ReadBytesExt;
use constants::ansi_term::Colour::*;

pub static STANDARD_COLOR: &'static str = "yellow";
pub static SUCCESS_COLOR: &'static str = "green";
pub static FAIL_COLOR: &'static str = "red";
pub static WALTON_DATA_COLOR: &'static str = "cyan";
pub static MING_DATA_COLOR: &'static str = "purple";
pub static AMOUNT_GPU: &'static i32 = &1;
pub static PORT_NUMBER_START: &'static i32 = &12140;

pub fn generate_randoms() -> Vec<i32> {
    let mut random_numbers: Vec<i32> = Vec::new();
    for x in 0..1000 {
        let num = rand::thread_rng().gen_range(100000, 1000000);
        random_numbers.push(num);
        println!("Generated {}", num);
    }
    return random_numbers;
}

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

pub fn print_44(packets: &Vec<u8>) {
    print_color("Formatted:", &MING_DATA_COLOR.to_owned());
    let unidentified_1 = packets.get(0..2).unwrap();
    let unidentified_2 = packets.get(2..4).unwrap();
    let input = packets.get(4..36).unwrap();
    let input_nonce = packets.get(36..44).unwrap();
    print_color(&format!("  Unidentified_1: {:?}", &unidentified_1.to_vec()),
                &STANDARD_COLOR.to_owned());
    print_color(&format!("  Unidentified_2: {:?}", &unidentified_2.to_vec()),
                &STANDARD_COLOR.to_owned());
    print_color(&format!("  Input Value   : {:?}", &input.to_vec()),
                &STANDARD_COLOR.to_owned());
    print_color(&format!("  Input Nonce   : {:?}", &input_nonce.to_vec()),
                &STANDARD_COLOR.to_owned());
}

pub fn print_96(packets: &Vec<u8>) {
    print_color("Formatted:", &WALTON_DATA_COLOR.to_owned());
    let block_number = packets.get(1..5).unwrap();  //Byte Index 1-4 - Index 0 = set/stop
    let count = packets.get(77..85).unwrap(); // Count is constant
    let input_nonce = packets.get(37..45).unwrap();
    let algtion_val = packets.get(85..96).unwrap();
    let input_val = packets.get(5..37).unwrap();
    let target_val = packets.get(45..77).unwrap();
    print_block(&block_number);
    print_color(&format!("  Count Val  : {:?}", &count.to_vec()),
                &SUCCESS_COLOR.to_owned());
    print_color(&format!("  Input Nonce: {:?}", &input_nonce.to_vec()),
                &SUCCESS_COLOR.to_owned());
    print_color(&format!("  Algtion Val: {:?}", &algtion_val.to_vec()),
                &SUCCESS_COLOR.to_owned());
    print_color(&format!("  Input   Val: {:?}", &input_val.to_vec()),
                &SUCCESS_COLOR.to_owned());
    print_color(&format!("  Target  Val: {:?}", &target_val.to_vec()),
                &SUCCESS_COLOR.to_owned());
}
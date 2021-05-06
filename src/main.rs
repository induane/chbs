extern crate clap;
use clap::{Arg, App};

use chbslib::get_entropy;


pub fn main() {
    let matches = App::new("CHBS Password Strength Checker")
        .version("0.1.0")
        .author("Brant Watson <oldspiceap@gmail.com>")
        .about("Checks a passwords strength and outputs a score")
        .arg(Arg::with_name("password").required(true))
        .get_matches();

    let password = matches.value_of("password").unwrap();
    println!("The password passed is: {}", password);
    let score = get_entropy(&password);
    println!("The score is: {}", score);
}

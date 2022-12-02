use clap::Parser;
use nom::character::complete::{line_ending, u64};
use nom::multi::{many1,separated_list1};
use nom::sequence::terminated;
use nom::error::VerboseError;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'd', long="day")]
    day: Option<i32>,
    input: String,
}

fn day1(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let elf_parser = many1(terminated(u64, line_ending::<_,VerboseError<_>>));
    let mut elves_parser = separated_list1(line_ending::<_,VerboseError<_>>, elf_parser);
    let elves = elves_parser(input_string.as_str()).unwrap().1;
    let mut calories = elves.iter().map(|items| { items.iter().sum::<u64>() }).collect::<Vec<_>>();
    println!("maximum: {}", calories.iter().max().unwrap());
    calories.sort();
    println!("top three sum: {}", calories.iter().rev().take(3).sum::<u64>());
}

fn main() {
    let args = Args::parse();
    match args.day {
        Some(1) => day1(args.input),
        Some(_) => println!("Invalid day selected"),
        None => println!("No day selected"),
    }
}

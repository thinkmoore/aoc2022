use clap::Parser;
use nom::branch::alt;
use nom::combinator::value;
use nom::character::complete::{char, line_ending, u64};
use nom::multi::{many1,separated_list1};
use nom::sequence::{separated_pair,terminated};
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

#[derive(Clone, Copy)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum RoundGoal {
    Lose,
    Draw,
    Win,
}

fn day2(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let opp_parser =  alt(
        (
            value(Throw::Rock, char('A')),
            value(Throw::Paper, char('B')),
            value(Throw::Scissors, char('C')),
        )
    );
    let my_parser = alt(
        (
            value(Throw::Rock, char('X')),
            value(Throw::Paper, char('Y')),
            value(Throw::Scissors, char('Z')),
        )
    );
    let mut parser = many1(
        terminated(
            separated_pair(opp_parser, char(' '), my_parser),
            line_ending::<_,VerboseError<_>>
        )
    );
    let compute_selection_score = |entry: &(Throw, Throw)| {
        match entry {
            (_, Throw::Rock) => 1,
            (_, Throw::Paper) => 2,
            (_, Throw::Scissors) => 3,
        }
    };
    let compute_game_score = |entry: &(Throw, Throw)| {
        match entry {
            (Throw::Rock, Throw::Rock) => 3,
            (Throw::Paper, Throw::Paper) => 3,
            (Throw::Scissors, Throw::Scissors) => 3,
            (Throw::Rock, Throw::Scissors) => 0,
            (Throw::Paper, Throw::Rock) => 0,
            (Throw::Scissors, Throw::Paper) => 0,
            (Throw::Rock, Throw::Paper) => 6,
            (Throw::Paper, Throw::Scissors) => 6,
            (Throw::Scissors, Throw::Rock) => 6,
        }
    };
    let parse = parser(input_string.as_str()).unwrap();
    let entries = parse.1;
    let selection_score: u64 = entries.iter().map(compute_selection_score).sum();
    let game_score: u64 = entries.iter().map(compute_game_score).sum();
    println!("Selection score: {selection_score}");
    println!("Game score: {game_score}");
    println!("Total: {}", game_score + selection_score);
    let reinterpreted : Vec<(Throw, RoundGoal)> =
        entries.iter().map(
            |entry| {
                (
                    entry.0,
                    match entry.1 {
                        Throw::Rock => RoundGoal::Lose,
                        Throw::Paper => RoundGoal::Draw,
                        Throw::Scissors => RoundGoal::Win,
                    }
                )
            }
        ).collect();
    let pick_throws = |entry: &(Throw,RoundGoal)| {
        let my_throw = match entry {
            (Throw::Rock, RoundGoal::Lose) => Throw::Scissors,
            (Throw::Paper, RoundGoal::Draw) => Throw::Paper,
            (Throw::Scissors, RoundGoal::Win) => Throw::Rock,
            (Throw::Rock, RoundGoal::Win) => Throw::Paper,
            (Throw::Paper, RoundGoal::Lose) => Throw::Rock,
            (Throw::Scissors, RoundGoal::Draw) => Throw::Scissors,
            (Throw::Rock, RoundGoal::Draw) => Throw::Rock,
            (Throw::Paper, RoundGoal::Win) => Throw::Scissors,
            (Throw::Scissors, RoundGoal::Lose) => Throw::Paper,
        };
        (entry.0, my_throw)
    };
    let revised: Vec<(Throw,Throw)> = reinterpreted.iter().map(pick_throws).collect();
    let new_selection_score: u64 = revised.iter().map(compute_selection_score).sum();
    let new_game_score: u64 = revised.iter().map(compute_game_score).sum();
    println!("Using revised throws");
    println!("Selection score: {new_selection_score}");
    println!("Game score: {new_game_score}");
    println!("Total: {}", new_game_score + new_selection_score);
}

fn main() {
    let args = Args::parse();
    match args.day {
        Some(1) => day1(args.input),
        Some(2) => day2(args.input),
        Some(_) => println!("Invalid day selected"),
        None => println!("No day selected"),
    }
}

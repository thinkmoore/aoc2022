use clap::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, anychar, char, digit1, line_ending, u64};
use nom::combinator::{map, value};
use nom::error::VerboseError;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use std::collections::HashSet;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'd', long = "day")]
    day: Option<i32>,
    input: String,
}

fn day1(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let elf_parser = many1(terminated(u64, line_ending::<_, VerboseError<_>>));
    let mut elves_parser = separated_list1(line_ending::<_, VerboseError<_>>, elf_parser);
    let elves = elves_parser(input_string.as_str()).unwrap().1;
    let mut calories = elves
        .iter()
        .map(|items| items.iter().sum::<u64>())
        .collect::<Vec<_>>();
    println!("maximum: {}", calories.iter().max().unwrap());
    calories.sort();
    println!(
        "top three sum: {}",
        calories.iter().rev().take(3).sum::<u64>()
    );
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
    let opp_parser = alt((
        value(Throw::Rock, char('A')),
        value(Throw::Paper, char('B')),
        value(Throw::Scissors, char('C')),
    ));
    let my_parser = alt((
        value(Throw::Rock, char('X')),
        value(Throw::Paper, char('Y')),
        value(Throw::Scissors, char('Z')),
    ));
    let mut parser = many1(terminated(
        separated_pair(opp_parser, char(' '), my_parser),
        line_ending::<_, VerboseError<_>>,
    ));
    let compute_selection_score = |entry: &(Throw, Throw)| match entry {
        (_, Throw::Rock) => 1,
        (_, Throw::Paper) => 2,
        (_, Throw::Scissors) => 3,
    };
    let compute_game_score = |entry: &(Throw, Throw)| match entry {
        (Throw::Rock, Throw::Rock) => 3,
        (Throw::Paper, Throw::Paper) => 3,
        (Throw::Scissors, Throw::Scissors) => 3,
        (Throw::Rock, Throw::Scissors) => 0,
        (Throw::Paper, Throw::Rock) => 0,
        (Throw::Scissors, Throw::Paper) => 0,
        (Throw::Rock, Throw::Paper) => 6,
        (Throw::Paper, Throw::Scissors) => 6,
        (Throw::Scissors, Throw::Rock) => 6,
    };
    let parse = parser(input_string.as_str()).unwrap();
    let entries = parse.1;
    let selection_score: u64 = entries.iter().map(compute_selection_score).sum();
    let game_score: u64 = entries.iter().map(compute_game_score).sum();
    println!("Selection score: {selection_score}");
    println!("Game score: {game_score}");
    println!("Total: {}", game_score + selection_score);
    let reinterpreted: Vec<(Throw, RoundGoal)> = entries
        .iter()
        .map(|entry| {
            (
                entry.0,
                match entry.1 {
                    Throw::Rock => RoundGoal::Lose,
                    Throw::Paper => RoundGoal::Draw,
                    Throw::Scissors => RoundGoal::Win,
                },
            )
        })
        .collect();
    let pick_throws = |entry: &(Throw, RoundGoal)| {
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
    let revised: Vec<(Throw, Throw)> = reinterpreted.iter().map(pick_throws).collect();
    let new_selection_score: u64 = revised.iter().map(compute_selection_score).sum();
    let new_game_score: u64 = revised.iter().map(compute_game_score).sum();
    println!("Using revised throws");
    println!("Selection score: {new_selection_score}");
    println!("Game score: {new_game_score}");
    println!("Total: {}", new_game_score + new_selection_score);
}

fn day3(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let mut parser = many1(terminated(alpha0, line_ending::<_, VerboseError<_>>));
    let rucksacks: Vec<(&str, &str)> = parser(input_string.as_str())
        .unwrap()
        .1
        .iter()
        .map(|sack: &&str| {
            (
                &sack[0..(sack.len() / 2)],
                &sack[(sack.len() / 2)..sack.len()],
            )
        })
        .collect();
    let find_duplicate = |entry: &(&str, &str)| {
        let left: HashSet<char> = HashSet::from_iter(entry.0.chars());
        let right: HashSet<char> = HashSet::from_iter(entry.1.chars());
        left.intersection(&right).next().unwrap().clone()
    };
    let duplicates: Vec<char> = rucksacks.iter().map(find_duplicate).collect();
    let to_priority = |item: &char| {
        let priority = if item.clone() as u64 - ('A' as u64) < 27 {
            item.clone() as u64 - ('A' as u64) + 27
        } else {
            item.clone() as u64 - ('a' as u64) + 1
        };
        println!("Item: {item}");
        println!("Priority: {priority}");
        priority
    };
    println!(
        "Sum of duplicate priorities: {}",
        duplicates.iter().map(to_priority).sum::<u64>()
    );
    let firsts = rucksacks.iter().step_by(3);
    let seconds = rucksacks.iter().skip(1).step_by(3);
    let thirds = rucksacks.iter().skip(2).step_by(3);
    let groups = firsts
        .zip(seconds)
        .zip(thirds)
        .map(|item| (item.0 .0, item.0 .1, item.1));
    let find_badge = |entry: (&(&str, &str), &(&str, &str), &(&str, &str))| {
        let first: HashSet<char> = HashSet::from_iter(entry.0 .0.chars().chain(entry.0 .1.chars()));
        let second: HashSet<char> =
            HashSet::from_iter(entry.1 .0.chars().chain(entry.1 .1.chars()));
        let third: HashSet<char> = HashSet::from_iter(entry.2 .0.chars().chain(entry.2 .1.chars()));
        first
            .intersection(&second)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&third)
            .next()
            .unwrap()
            .clone()
    };
    let badges: Vec<char> = groups.map(find_badge).collect();
    println!(
        "Sum of badge priorities: {}",
        badges.iter().map(to_priority).sum::<u64>()
    );
}

fn day4(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let mut parser = separated_list1(
        line_ending::<_, VerboseError<_>>,
        separated_pair(
            separated_pair(u64, char('-'), u64),
            char(','),
            separated_pair(u64, char('-'), u64),
        ),
    );
    let input: Vec<((u64, u64), (u64, u64))> = parser(input_string.as_str()).unwrap().1;
    let fully_overlaps = |entry: &&((u64, u64), (u64, u64))| {
        ((entry.0 .0 <= entry.1 .0) && (entry.0 .1 >= entry.1 .1))
            || ((entry.0 .0 >= entry.1 .0) && (entry.0 .1 <= entry.1 .1))
    };
    println!(
        "Fully overlaps: {}",
        input.iter().filter(fully_overlaps).count()
    );
    let overlaps = |entry: &&((u64, u64), (u64, u64))| {
        ((entry.0 .0 <= entry.1 .0) && (entry.1 .0 <= entry.0 .1))
            || ((entry.0 .0 <= entry.1 .1) && (entry.1 .1 <= entry.0 .1))
            || ((entry.1 .0 <= entry.0 .0) && (entry.0 .0 <= entry.1 .1))
            || ((entry.1 .0 <= entry.0 .1) && (entry.0 .1 <= entry.1 .1))
    };
    println!("Overlaps: {}", input.iter().filter(overlaps).count())
}

fn day5(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let crate_id = map(delimited(char('['), anychar, char(']')), |c: char| Some(c));
    let missing = value(None, tuple((char(' '), char(' '), char(' '))));
    let row = separated_list1(char(' '), alt((crate_id, missing)));
    let rows = many1(terminated(row, line_ending::<_, VerboseError<_>>));
    let ids = terminated(
        separated_list1(char(' '), delimited(char(' '), digit1, char(' '))),
        many1(line_ending::<_, VerboseError<_>>),
    );
    let inst = tuple((
        preceded(tag("move "), u64),
        preceded(tag(" from "), u64),
        preceded(tag(" to "), u64),
    ));
    let insts = separated_list1(line_ending::<_, VerboseError<_>>, inst);
    let mut parser = tuple((rows, ids, insts));
    let result = parser(input_string.as_str());
    match result {
        Ok((rem, _)) => println!("Parsing successful, remaining text is:\n{}", rem),
        Err(nom::Err::Error(ref error)) => println!("Parsing failed: {}", error),
        Err(nom::Err::Failure(ref error)) => println!("Parsing failed: {}", error),
        _ => unreachable!(),
    };
    let input: (Vec<Vec<Option<char>>>, Vec<&str>, Vec<(u64, u64, u64)>) = result.unwrap().1;
    let mut boxes: Vec<Vec<char>> = vec![Vec::new(); &input.1.len() + 1];
    let mut more_boxes = boxes.clone();
    for row in input.0.iter().rev() {
        for i in 0..row.len() {
            match row[i] {
                Some(c) => boxes[i + 1].push(c),
                None => (),
            }
        }
    }
    for i in 1..boxes.len() {
        print!("Box {i}: ");
        for c in &boxes[i] {
            print!("{c} ");
        }
        println!("");
    }
    for (count, start, end) in &input.2 {
        println!("move {} from {} to {}", count, start, end);
        for _ in 0..*count {
            match boxes[*start as usize].pop() {
                Some(c) => boxes[*end as usize].push(c),
                None => (),
            }
        }
        for i in 1..boxes.len() {
            print!("Box {i}: ");
            for c in &boxes[i] {
                print!("{c} ");
            }
            println!("");
        }
    }
    print!("Top boxes: ");
    for i in 1..boxes.len() {
        print!(
            "{}",
            match boxes[i].pop() {
                Some(c) => c,
                None => '*',
            }
        );
    }
    println!("");
    println!("Starting again with the CrateMover 9001...");
    for row in input.0.iter().rev() {
        for i in 0..row.len() {
            match row[i] {
                Some(c) => more_boxes[i + 1].push(c),
                None => (),
            }
        }
    }
    for i in 1..more_boxes.len() {
        print!("Box {i}: ");
        for c in &more_boxes[i] {
            print!("{c} ");
        }
        println!("");
    }
    for (count, start, end) in &input.2 {
        println!("move {} from {} to {}", count, start, end);
        let slice_start: usize = more_boxes[*start as usize].len() - (*count as usize);
        let mut to_place: Vec<char> = more_boxes[*start as usize].drain(slice_start..).collect();
        more_boxes[*end as usize].append(&mut to_place);
        for i in 1..more_boxes.len() {
            print!("Box {i}: ");
            for c in &more_boxes[i] {
                print!("{c} ");
            }
            println!("");
        }
    }
    print!("Top boxes (CrateMover 9001): ");
    for i in 1..more_boxes.len() {
        print!(
            "{}",
            match more_boxes[i].pop() {
                Some(c) => c,
                None => '*',
            }
        );
    }
    println!("");
}

fn day6(input_file: String) {
    let input_string = fs::read_to_string(input_file).unwrap();
    let mut offset = 0;
    for i in 0..input_string.len() {
        let chars: HashSet<char> = HashSet::from_iter(input_string[i..i + 4].chars());
        if chars.len() == 4 {
            offset = i + 4;
            break;
        }
    }
    println!("Packet starts at: {}", offset);
    offset = 0;
    for i in 0..input_string.len() {
        let chars: HashSet<char> = HashSet::from_iter(input_string[i..i + 14].chars());
        if chars.len() == 14 {
            offset = i + 14;
            break;
        }
    }
    println!("Packet starts at: {}", offset);
}

fn main() {
    let args = Args::parse();
    match args.day {
        Some(1) => day1(args.input),
        Some(2) => day2(args.input),
        Some(3) => day3(args.input),
        Some(4) => day4(args.input),
        Some(5) => day5(args.input),
        Some(6) => day6(args.input),
        Some(_) => println!("Invalid day selected"),
        None => println!("No day selected"),
    }
}

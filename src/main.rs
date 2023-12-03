use std::env;
mod commons;
mod day1;
mod day2;

fn main() -> std::io::Result<()> {
    let mut day_to_run: i32 = -1; // -1 means latest
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        day_to_run = args[1].parse::<i32>().unwrap();
    } else {
        println!("Missing argument: Day, Will run: Latest");
    }

    match day_to_run {
        1 => day1::day1(),
        2 => day2::day2(),
        -1 => day2::day2(),
        _ => println!("Invalid day_to_run value"),
    }
    Ok(())
}

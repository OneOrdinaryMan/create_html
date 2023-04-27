use std::{env, fs, path::Path, process};
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("There are no arguments!");
        process::exit(1);
    } else if 1 < args.len() {
        println!("There are more than one arguments");
    }
    let home_env = option_env!("HOME");
    let home_dir = match home_env {
        Some(value) => String::from(value),
        None => String::new(),
    };
    if home_dir.is_empty() {
        println!("The env is not set");
        process::exit(1);
    }
    let html_dir = format!("{}{}", &home_dir, "/Development/HTML");
    let html_path = Path::new(&html_dir);
    if !html_path.try_exists().unwrap() {
        println!("The path doesnt exists");
        process::exit(1);
    }
    fs::create_dir(&html_path.join(&args[0])).unwrap();
}

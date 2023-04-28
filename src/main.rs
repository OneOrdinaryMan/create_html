use std::{
    env, fs,
    io::{stdin, Write},
    path::Path,
    process,
};
fn file_contents() -> String {
    let mut contents: String = String::new();
    contents.push_str("<!Doctype=HTML>\n<head>\n");
    contents.push_str("<html>\n");
    let mut title = String::new();
    println!("Input the title here!");
    stdin().read_line(&mut title).unwrap();
    title.insert_str(0, "<title> ");
    title = String::from(title.trim_end());
    title.push_str(" </title>");
    contents.push_str(&title);
    contents.push_str("\n</head>\n");
    contents.push_str("<body>\nHello\n</body>\n");
    contents.push_str("</html>");
    contents
}

fn check_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("There are no arguments!");
        process::exit(1);
    } else if 1 < args.len() {
        println!("There are more than one arguments");
        process::exit(1);
    }
    args[0].to_string()
}

fn main() {
    let args = check_args();
    let write_string = file_contents();
    let home_dir = match option_env!("HOME") {
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
    fs::create_dir(&html_path.join(&args)).unwrap();
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&html_path.join(args.to_owned() + "/index.html"))
        .unwrap();
    file.write_all(write_string.as_bytes()).unwrap();
}

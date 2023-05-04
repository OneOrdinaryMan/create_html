use std::{
    env, fs,
    io::{stdin, Result, Write},
    path::{Path, PathBuf},
    process,
};
fn file_contents() -> Result<String> {
    let mut contents: String = String::new();
    contents.push_str("<!Doctype=HTML>\n<html>\n");
    contents.push_str("<head>\n");
    let mut title = String::new();
    println!("Input the title here!");
    stdin().read_line(&mut title)?;
    title.insert_str(0, "<title> ");
    title = String::from(title.trim_end());
    title.push_str(" </title>");
    contents.push_str(&title);
    contents.push_str("\n</head>\n");
    contents.push_str("<body>\nHello\n</body>\n");
    contents.push_str("</html>");
    Ok(contents)
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

fn check_env() -> String {
    let home_dir = match option_env!("HOME") {
        Some(value) => String::from(value),
        None => String::new(),
    };
    if home_dir.is_empty() {
        println!("The env is not set");
        process::exit(1);
    }
    home_dir
}

fn html_dir_path(home_str: String) -> Result<PathBuf> {
    let html_dir = format!("{}{}", &home_str, "/Development/HTML");
    let html_path = Path::new(&html_dir);

    if !html_path.try_exists()? {
        println!("The path doesnt exists");
        process::exit(1);
    }
    Ok(html_path.to_path_buf())
}

fn create_files(html_path: PathBuf, args: String, write_string: String) -> Result<()> {
    fs::create_dir(&html_path.join(&args))?;
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&html_path.join(args.to_owned() + "/index.html"))?;
    file.write_all(write_string.as_bytes())?;
    Ok(())
}
fn main() {
    let args = check_args();
    let write_string = match file_contents() {
        Ok(value) => value,
        Err(e) => {
            println!("Error found here: {}", e);
            String::new()
        }
    };
    let home_dir = check_env();
    match html_dir_path(home_dir) {
        Ok(html_path) => match create_files(html_path, args, write_string) {
            Ok(()) => println!("Success! file created"),
            Err(e) => println!("Failed due to an error {}", e),
        },
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

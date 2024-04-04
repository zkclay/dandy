use std::env;

mod addr;

fn parse_case_sensitive(arg: &str) -> bool {
    matches!(arg.to_lowercase().as_str(), "true" | "t" | "yes" | "y" | "1")
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // Check if we got the correct number of arguments (4 in total, including the program name)
    if args.len() < 4 {
        eprintln!("Usage: {} <prefix> <caseSensitive?> <resultFilePath>", args[0]);
        std::process::exit(1);
    }

    let prefix = &args[1];
    let case_sensitive = parse_case_sensitive(&args[2]);
    let file_path = &args[3];

    addr::find_vanity_address(prefix.to_string(), case_sensitive, file_path);
}
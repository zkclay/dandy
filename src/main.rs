
mod addr;

fn main() {
    let prefix = String::from("q");
    let case_sensitive = false;
    let file_path = "results.txt";
    addr::find_vanity_address(prefix, case_sensitive, file_path);
}
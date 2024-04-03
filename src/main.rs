mod addr;

fn main() {
    let prefix = String::from("zk");
    let case_sensitive = false; 

    loop {
        let result = addr::generate_addr();
        let addr_string = if case_sensitive {
            result.1.clone().to_lowercase()
        } else {
            result.1.clone()
        };
    
        if addr_string.to_lowercase().starts_with(&prefix) {
            println!("{:?}", &result);
        }
    }
}
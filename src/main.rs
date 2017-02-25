use std::path::Path;
use std::io::Write;

fn main() {
    init_mos();
}

fn init_mos() {
    let path = Path::new("me.mos");

    if path.exists() {
        println!("File exists: me.mos");
    }
    else {
        first_use();
    }
}

fn first_use() {
    let mut input: String = String::new();
    
    println!("First usage of mos, please fill in user credentials:");
    print!("Nickname: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input);
}

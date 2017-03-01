use std::path::Path;
use std::io::Write;
use std::env;

fn main() {
    init_mos();
}

fn init_mos() {
    let exe_curr = env::current_exe().unwrap();
    let exe_path_parent = exe_curr.as_path().parent().unwrap();
    
    let me_path = format!("{}/{}", exe_path_parent.display(), "me.mos");
    
    let path = Path::new(me_path.as_str());

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
    print!("First name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input);
    print!("Surname: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input);
    print!("Profession: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input);
}

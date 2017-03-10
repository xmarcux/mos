mod user;

use std::path::Path;
use std::io::{ Write, BufWriter };
use std::env;
use std::fs;
use user::{ Available, User };

fn main() {
    init_mos();
}

fn init_mos() {
    let exe_curr = env::current_exe().unwrap();
    let exe_path_parent = exe_curr.as_path().parent().unwrap();
    
    let me_path = format!("{}/mosdb/{}", exe_path_parent.display(), "me.mos");
    
    let path = Path::new(me_path.as_str());

    if path.exists() {
        println!("File exists: me.mos");
    }
    else {
        match first_use() {
            Err(msg) => println!("{}", msg),
            _ => (),
        }
    }
}

fn first_use() -> Result<(), String>{
    let exe_curr = env::current_exe().unwrap();
    let exe_path_parent = exe_curr.as_path().parent().unwrap();
    let db_path_str = format!("{}/{}", exe_path_parent.display(), "mosdb");
    let db_path = Path::new(db_path_str.as_str());
    
    let mut u = User { nickname: "".to_string(), name: "".to_string(), surname: "".to_string(), profession: "".to_string(), available: Available::Available };

    if !db_path.is_dir() {
        let dir_res = fs::create_dir(db_path.to_str().unwrap());

        if dir_res.is_err() {
            return Err("Error creating directory: mosdb".to_string())
        }
    }
    
    println!("First usage of mos, please fill in user credentials:");
    print!("Nickname: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut u.nickname);
    u.nickname = u.nickname.trim().to_string();
    print!("First name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut u.name);
    u.name = u.name.trim().to_string();
    print!("Surname: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut u.surname);
    u.surname = u.surname.trim().to_string();
    print!("Profession: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut u.profession);
    u.profession = u.profession.trim().to_string();

    let me_f_res = fs::File::create(format!("{}/me.mos", db_path_str));

    if me_f_res.is_ok() {
        let mut me_f_buf = BufWriter::new(me_f_res.unwrap());
        let w_res = me_f_buf.write_all(u.to_frm_str().as_bytes());

        if w_res.is_err() {
            return Err(format!("Error writing to file: {}/me.mos", db_path_str));
        }
    }
    else {
        return Err(format!("Error creating file: {}/me.mos", db_path_str))
    }
    
    Ok(())
}

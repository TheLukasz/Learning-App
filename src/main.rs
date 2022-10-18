use std::fs::File;
use std::io::Read;
use std::io;
use colored::Colorize;
use std::path::Path;


fn main() {
    // Initializing geting input from the user
    let mut user_input = String::new();
    let stdin = io::stdin();

    // Opening file
    let f = File::options().append(true).read(true).create(true)
            .open(match std::env::args().nth(1){
                Some(path) => {
                    match Path::new(&path).exists() {
                        true => path,
                        false => {
                            println!("Wrong file name provided as env variable");
                            return
                            }
                    }
                }
                None => {
                    println!("No file name provided, pleas enter the name of a file.");
                    stdin.read_line(&mut user_input).unwrap();
                    match Path::new(user_input.trim()).exists() {
                        true => user_input.trim().to_string(),
                        false => {
                            println!("No file with such name");
                            return
                            }
                    }
                },
            }
    );

    let mut f = match f {
        Ok(file) => file,
        Err(_err) => panic!("PermissionDenied"),
    };

    let mut data = String::new();
    f.read_to_string(&mut data).expect("Unable to read string");
    if data.len() == 0 {
        println!("The file is is empty.");
        return
    }


    let mut list:Vec<[&str;2]> = Vec::new();
    for line in data.lines() {
        list.push(line.split("=").map(str::trim).collect::<Vec<&str>>().try_into().unwrap());
    }
    let mut tmp_list = list.clone();
    let mut i:usize;
    // Main loop
    loop {
        while tmp_list.is_empty() == false {
            i = rand::random::<usize>()%tmp_list.len();
            println!("{}",tmp_list[i][0]);
            user_input.clear();
            stdin.read_line(&mut user_input).unwrap();
            let to_match = user_input.trim().to_lowercase();
            if to_match == "quit" || to_match == ":q" {
                return;
            }
            else if to_match == tmp_list[i][1].to_lowercase() {
                println!("{}\n","Dobra odpowiedz".green());
                tmp_list.remove(i);
            }
            else {
                println!("{} poprawna odpowiedz: {} - {}","Zla odpowiedz".red(),tmp_list[i][0].blue(),tmp_list[i][1].blue());
            }
        }
        tmp_list = list.clone();
        println!("----------------------------------------------------");
    }
}
    /*let mut tmp:Vec<&str> = data.split("\n").collect::<Vec<&str>>();
    tmp.retain(|&x| x != "");
    for p in  tmp {
        let mut vec:Vec<&str> = p.split("-").collect();
        for v in &mut vec {
            *v = v.trim();
        }
        list.push(vec.try_into().unwrap());
    }*/

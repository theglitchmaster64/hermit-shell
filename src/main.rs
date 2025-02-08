#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use hex;
use rpassword::read_password;


fn main() {
    let mut shell_state = ShellState::new();

    shell_state.set_env_var("USER".to_string(), "guest".to_string());

    println!("\n---ENVIRONMENT---\n");
    shell_state.print_shell_state();

    loop {
        // print prompt >
        print!("{} > ", shell_state.current_dir.to_string_lossy());
        match std::io::stdout().flush() {
            Ok(_) => {
                ();
            }
            Err(err) => {
                print!("STDOUT ERROR! {}", err);
            }
        }

        // read user input
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                ();
            } //println!("input = {}",input.trim());}
            Err(err) => {
                println!("STDIN ERROR! {}", err);
            }
        }

        // vectorize input
        let input_vec: Vec<&str> = input.split_whitespace().collect();
        eprintln!("DEBUG: {:?}", input_vec);
        if input_vec.is_empty() {
            continue;
        }




        // shell builtins
        match input_vec[0].to_lowercase().as_str() {
            "login" => {
                println!("USERNAME: ");
                let mut username = String::new();
                std::io::stdin().read_line(&mut username).unwrap();
                println!("PASSWORD: ");
                let password = read_password().unwrap();
                let secret_hash = "567fd210681d2f6c9da0ae174ca8303f94348d6c7619fcde18bb953751d4682f";
                let to_hash = format!("{}@@{}",username.trim(),password.trim());
                //println!("{}",to_hash);
                let mut hasher = Sha256::new();
                hasher.update(&to_hash);
                let result = hex::encode(hasher.finalize());
                //println!("{}\n{}\n{}", result, secret_hash, to_hash);
                if result == secret_hash {
                    shell_state.logged_in = true;
                    shell_state.set_env_var("USER".to_string(), username.to_string());
                    println!("logged in successfully as {}!", username.trim())
                }
                else {
                    println!("wrong credentials!")
                }
            }
            "logout" => {
                shell_state.set_env_var("USER".to_string(), "guest".to_string());
                shell_state.logged_in = false;
                println!{"successfully logged out!"}
            }
            "help" => {
                println!("SHELL BUILTINS:\nlogin|logout|help|status|test|exit|echo|cd");
            }
            "status" => {
                println!("\n---ENVIRONMENT---\n");
                shell_state.print_shell_state();
            }
            "test" => {
                println!("supm8");
            }
            "exit" | "quit" => {
                break;
            }
            "echo" => {
                echo(&input_vec[1..].join(" "));
            }
            "cd" => {
                if input_vec.len() >= 2 {
                    shell_state.change_directory(input_vec[1]);
                } else {
                    eprintln!("NO PATH SPECIFIED!");
                }
            }
            _ => {
                println!("command not found! try 'help' to get a list of shell builtins");
                continue;
            }
        }
    }
}



struct ShellState {
    env_vars: HashMap<String, String>,
    current_dir: PathBuf,
    logged_in: bool,
}



impl ShellState {
    fn new() -> Self {
        Self {
            env_vars: HashMap::new(),
            current_dir: std::env::current_dir().unwrap(),
            logged_in: false,
        }
    }

    fn set_env_var(&mut self, key: String, value: String) {
        self.env_vars.insert(key, value);
    }

    fn print_shell_state(&self) {
        println!("LOGGED_IN:{}", &self.logged_in);
        for (key, value) in &self.env_vars {
            println!("{}:{}", key, value);
        }
        println!("\n-----------------");
    }

    fn change_directory(&mut self, new_dir: &str) {
        let path = std::path::Path::new(new_dir);
        if path.exists() {
            self.current_dir = path.to_path_buf();
        } else {
            eprintln!("INVALID PATH!")
        }
    }
}

fn echo(echo_str: &str) {
    println!("{}", echo_str);
}

use std::{process::exit, env::args, io::{Read, stdin}, fs::File};

fn main() {
    let mut args = args().skip(1);
    let length = args.len();

    if length < 1 {
        eprintln!("grepw: Pattern not specified");
        exit(1);
    }

    let mut case_insensitive = false;
    let mut inverse_match = false;
    let mut file = String::new();

    let pattern = args.next().unwrap();

    while let Some(arg) = args.next() {
        let mut iter = arg.chars();
        if iter.next() == Some('-') {
            while let Some(ch) = iter.next() {
                match ch {
                    '-' => {
                        match arg.as_str() {
                            "--case-insensitive" | "--insensitive" => case_insensitive = true,
                            "--inverse-match" | "--inverse" => inverse_match = true,
                            _ => {
                                eprintln!("grepw: Unknown command-line option {arg}");
                                exit(1);
                            }
                        }
                    }
                    'i' => {
                        case_insensitive = true;
                        continue;
                    }
                    'I' => {
                        inverse_match = true;
                        continue;
                    }
                    'h' => {
                        eprintln!("Usage: grepw PATTERN [OPTIONS] FILEPATH");
                        continue;
                    }
                    _ => {
                        eprintln!("grepw: Unknown command-line option -{ch}");
                    }
                }
            }
        } else {
            file = arg;
            break;
        }
    }

    if file.as_str() != "" {
        let res = File::open(&file);
        if let Ok(mut file) = res {
            let mut buffer = String::new();
            let res = file.read_to_string(&mut buffer);
            if let Ok(_) = res {
                let lines = buffer.split("\n");
                if case_insensitive {
                    if inverse_match {
                        for line in lines {
                            if !line.to_lowercase().contains(&pattern.to_lowercase()) {
                                println!("{}", line);
                            }
                        }
                    } else {
                        for line in lines {
                            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                                println!("{}", line);
                            }
                        }
                    }
                } else {
                    if inverse_match {
                        for line in lines {
                            if !line.contains(&pattern) {
                                println!("{}", line);
                            }
                        }
                    } else {
                        for line in lines {
                            if line.contains(&pattern) {
                                println!("{}", line);
                            }
                        }
                    }
                }
            } else if let Err(e) = res {
                eprintln!("grepw: Couldn't read file to string: {e}");
                exit(1);
            }
        } else if let Err(e) = res {
            eprintln!("grepw: Couldn't open file `{file}`: {e}");
            exit(1);
        }
    } else {
        let mut line = String::new();
        let res = stdin().read_line(&mut line);
        if let Ok(_) = res {
            if case_insensitive {
                if inverse_match {
                    if !line.to_lowercase().contains(&pattern.to_lowercase()) {
                        println!("{}", line);
                    }
                } else {
                    if line.to_lowercase().contains(&pattern.to_lowercase()) {
                        println!("{}", line);
                    }
                }
            } else {
                if inverse_match {
                    if !line.contains(&pattern) {
                        println!("{}", line);
                    }
                } else {
                    if line.contains(&pattern) {
                        println!("{}", line);
                    }
                }
            }
        } else if let Err(e) =  res {
            eprintln!("grepw: Couldn't read from stdin: {e}");
            exit(1);
        }
    }
}
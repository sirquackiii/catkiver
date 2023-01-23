use std::io;
use std::fs;
use std::io::Write;
use std::process::Command;

// CATKIVER
// archiver for cat images

#[derive(Debug, Clone)]
struct date {
    day: u8,
    month: u8,
    year: i64,
}

#[derive(Debug, Clone)]
struct file_entry {
    filename: String,
    donator: String,
    desc: String,
    hash: String,
    date: date,
}

// fn add_entry(date: date, donator: String, desc: String)

fn parse_ck(input: String) -> Vec<file_entry> {
    let mut buf: Vec<file_entry> = Vec::new();
    let data = input.split('\n');

    for e in data {
        let split: Vec<&str> = e.split(';').collect();
        let out = file_entry {
            filename: split[0].to_string(),
            donator: split[1].to_string(),
            desc: split[2].to_string(),
            hash: split[6].to_string(),
            date: date {
                day: split[3].parse::<u8>().unwrap(),
                month: split[4].parse::<u8>().unwrap(),
                year: split[5].parse::<i64>().unwrap(),
            },
        };
        buf.push(out);
    }

    buf
}

fn parse_str(inp: Vec<&str>) -> String {
    let mut out = String::new();

    let mut is_str: bool = false;

    for e in inp {
        if e.contains("\"") && !is_str {
            out.push_str(&e.replace("\"", ""));
            is_str = true;
        } else if e.contains("\"") && is_str {
            out.push_str(&e.replace("\"", ""));
            is_str = false;
        } else if is_str {
            out.push_str(&e);
        } else if !is_str {
            break;
        }

        out.push(' ');
    }

    out.trim().to_string()
}

fn main() {
    let path = "test.ck";

    let header: String = fs::read_to_string(path)
        .expect("Could not read file.");

    let dir = fs::read_to_string("home.txt")
        .expect("Could not read file.");

    let entries: Vec<file_entry> = parse_ck(header);

    let mut loaded_entry: Option<file_entry> = None;

    println!("{:#?}", entries);

    loop {
        // actually receive the command
        print!(">>> ");

        // do the thing
        std::io::stdout()
            .flush()
            .unwrap();

        let mut command = String::new();
        
        std::io::stdin()
            .read_line(&mut command)
            .expect("Could not receive input.");

        let data: Vec<&str> = command.split(' ').collect();

        match data[0].trim() {
            "exit" => {
                println!("Cat");
                break;
            },

            "search" => {
                let mut found: Vec<file_entry> = Vec::new();

                match data[1].trim() {
                    "donator" => {
                        for e in entries.clone() {
                            if e.donator.contains(data[2].trim()) {
                                found.push(e.clone());
                            }
                        }
                    },

                    "desc" => {
                        for e in entries.clone() {
                            let string = parse_str(data[2..].to_vec()).to_lowercase();

                            if e.desc
                                .to_lowercase()
                                .contains(&string) {
                                found.push(e.clone());
                            }
                        }
                    },

                    "hash" => {
                        for e in entries.clone() {
                            if e.hash.contains(data[2].trim()) {
                                found.push(e.clone());
                            }
                        }
                    },

                    "filename" => {
                        for e in entries.clone() {
                            if e.filename.contains(data[2].trim()) {
                                found.push(e.clone());
                            }
                        }
                    },

                    _ => println!("Unknown argument: {}", data[1])
                }

                match found.len() {
                    0 => println!("No results found."),
                    1 => println!("Found 1 result: "),
                    _ => println!("Found {} results: ", found.len())
                }

                println!("{:#?}", found);
            },

            "load" => {
                match data[1].trim() {
                    "hash" => {
                        for e in entries.clone() {
                            if e.hash.contains(data[2].trim()) {
                                loaded_entry = Some(e.clone());
                                break;
                            }
                        }
                    },

                    _ => println!("Unknown argument: {}", data[1])
                }
            },

            "open" => {
                if loaded_entry.is_none() {
                    println!("No entry loaded.");
                } else {
                    let entry = loaded_entry.clone().unwrap();

                    let path = format!("{}/{}", dir, entry.filename);

                    Command::new("feh")
                        .arg(path)
                        .spawn()
                        .expect("Could not open file.");
                }
            }

            _ => println!("Unknown command: {}", data[0]),
        }
    }
}

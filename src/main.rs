use hex;
use std::io;
use std::fs;
use std::io::Write;
use std::os::unix::fs::chroot;
use chrono::Datelike;
use std::process::Command;
use sha2::{Sha256, Digest};

// CATKIVER
// archiver for cat images

#[derive(Debug, Clone)]
struct date {
    day: u8,
    month: u8,
    year: u64,
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

    if input.len() == 0 {
        return buf;
    }

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
                year: split[5].parse::<u64>().unwrap(),
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
    let path = "entries.ck";

    let header: String = fs::read_to_string(path)
        .expect("Could not read file.");

    let dir = fs::read_to_string("home.txt")
        .expect("Could not read file.");

    let mut entries: Vec<file_entry> = parse_ck(header);

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

                    let path = format!("{}/{}", &dir, entry.filename);

                    Command::new("feh")
                        .arg(path)
                        .spawn()
                        .expect("Could not open file.");
                }
            }

            "new" => {
                if data[1] == "url" {
                    let mut buf: file_entry = file_entry {
                        filename: String::new(),
                        donator: String::new(),
                        desc: String::new(),
                        hash: String::new(),
                        date: date {
                            day: 0,
                            month: 0,
                            year: 0,
                        },
                    };
                    let url = data[2].trim();

                    // get filename
                    let filename = url.split('/').last().unwrap().trim();

                    let punt = format!("{}/{}", &dir, &filename);
                    let path = punt.trim();

                    // get image from url
                    Command::new("wget")
                        .arg(url)
                        .arg("-O")
                        .arg(path)
                        .output()
                        .expect("Could not download file.");

                    // get hash

                    println!("Getting hash...");

                    std::io::stdout()
                        .flush()
                        .unwrap();

                    let file_data = fs::read(path)
                        .expect("Could not read file.");

                    let mut hasher = Sha256::new();
                    hasher.update(&file_data);

                    let hash = hex::encode(hasher.finalize());

                    println!("{}", hash);

                    // get date
                    let time = chrono::Local::now();
                    let year = time.year();
                    let month = time.month();
                    let day = time.day();

                    // get description
                    let mut desc = String::new();

                    print!("Description: ");

                    std::io::stdout()
                        .flush()
                        .unwrap();

                    std::io::stdin()
                        .read_line(&mut desc)
                        .expect("Could not receive input.");

                    // get donator

                    let mut donator = String::new();

                    print!("Donator: ");

                    std::io::stdout()
                        .flush()
                        .unwrap();

                    std::io::stdin()
                        .read_line(&mut donator)
                        .expect("Could not receive input.");


                    buf.filename = filename.to_string();
                    buf.donator = donator.trim().to_string();
                    buf.desc = desc.trim().to_string();
                    buf.hash = hash;
                    buf.date = date {
                        day: day as u8,
                        month: month as u8,
                        year: year as u64,
                    };

                    entries.push(buf.clone());

                    println!("{:#?}", buf);
                } else {
                    println!("Unknown argument: {}", data[1]);
                }
            },

            "save" => {
                let mut out = String::new();

                for e in entries.clone() {
                    out.push_str(format!(
                        "{};{};{};{};{};{};{}\n",
                        e.filename, e.donator, e.desc, e.date.day, e.date.month, e.date.year, e.hash,)
                        .as_str());
                }

                fs::write("entries.ck", out)
                    .expect("Could not write to file.");
            }

            _ => println!("Unknown command: {}", data[0]),
        }
    }
}

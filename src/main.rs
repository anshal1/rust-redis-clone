use std::{
    collections::HashMap,
    fmt,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    str::FromStr,
};
#[derive(Debug)]
enum Command {
    GET,
    Add,
    DELETE,
}

#[derive(Debug, PartialEq)]
struct ParseCommandError(String); // keeps the bad input for error messages

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown command: {}", self.0)
    }
}

impl std::error::Error for ParseCommandError {}

impl FromStr for Command {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        if let Ok(comm) = get_command(&s.to_string()) {
            return match comm.as_str() {
                "add" => {
                    if split.len() < 3 {
                        return Err(ParseCommandError(
                            "Add command must have a key and atleast one value in double quoted"
                                .to_string(),
                        ));
                    }
                    let values = split[2..].iter();
                    let mut is_start_orrect = false;
                    let mut is_end_correct = false;

                    for val in values {
                        if val.starts_with('"') && !is_start_orrect {
                            is_start_orrect = true;
                        }
                        if val.ends_with('"') && !is_end_correct {
                            is_end_correct = true;
                        }
                    }
                    if !is_start_orrect || !is_end_correct {
                        return Err(ParseCommandError(
                            "Values must start and end with double quotes".to_string(),
                        ));
                    }
                    return Ok(Command::Add);
                }
                "get" => {
                    if split.len() != 2 {
                        return Err(ParseCommandError(
                            "No key provieded for get operation".to_string(),
                        ));
                    }
                    return Ok(Command::GET);
                }
                "del" => {
                    if split.len() != 2 {
                        return Err(ParseCommandError(
                            "No key provieded for delete operation".to_string(),
                        ));
                    }
                    return Ok(Command::DELETE);
                }
                _ => Err(ParseCommandError("Invalid command".to_string())),
            };
        } else {
            return Err(ParseCommandError("Invalid Command".to_string()));
        };
    }
}
fn get_command(input: &String) -> Result<String, String> {
    let split: Vec<&str> = input.split(" ").collect();
    if split.len() < 1 {
        return Err("Invalid Command".to_string());
    }

    Ok(String::from(split[0]))
}
fn get_values_and_key(input: &String) -> (String, String) {
    let mut string: String = String::new();
    let split: Vec<&str> = input.split(" ").collect();
    // means use hasn't provied any values after the command
    if split.len() == 2 {
        return (String::from(split[1]), string);
    }
    for values in split[1..].iter() {
        string.push_str(values);
        string.push(' ');
    }
    return (String::from(split[1]), string);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = BufReader::new(&stream);
        let mut string = String::new();
        buf.read_line(&mut string).unwrap();
        let string = string.trim().replace("\n", "");
        println!("{}", string);
        match string.parse::<Command>() {
            Ok(Command::Add) => {
                let (key, value) = get_values_and_key(&string);
                map.insert(key, value);
                stream.write_all("Success\n".as_bytes()).unwrap();
            }
            Ok(Command::GET) => {
                let (key, _) = get_values_and_key(&string);
                let val = if let Some(v) = map.get(&key) {
                    let mut newVal = String::new();
                    newVal.push_str(v);
                    newVal.push('\n');

                    String::from(newVal)
                } else {
                    String::from("None\n")
                };

                stream.write_all(val.as_bytes()).unwrap();
            }
            Ok(Command::DELETE) => {
                let (key, _) = get_values_and_key(&string);
                if let Some(_) = map.get(&key) {
                    match map.remove(&key) {
                        Some(_) => stream.write_all("Key removed".as_bytes()).unwrap(),
                        None => {
                            stream.write_all("No key found".as_bytes()).unwrap();
                        }
                    }
                } else {
                    stream.write_all("No key found".as_bytes()).unwrap();
                };
            }
            Err(e) => {
                stream.write_all(e.0.as_bytes()).unwrap();
            }
        }
    }
}

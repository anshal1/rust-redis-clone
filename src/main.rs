use std::{
    collections::HashMap,
    fmt,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
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
    for values in split[2..].iter() {
        string.push_str(values);
        string.push(' ');
    }
    return (String::from(split[1]), string);
}

fn write_stream(stream: &mut TcpStream, to_write: &str) {
    if let Err(e) = stream.write_all(to_write.as_bytes()) {
        println!("Unable to write to the stream: {}", e);
    } else {
        println!("Stream Write Success")
    }
}

fn perform_commands(string: String, mut stream: TcpStream, map: &mut HashMap<String, String>) {
    match string.parse::<Command>() {
        Ok(Command::Add) => {
            let (key, value) = get_values_and_key(&string);
            map.insert(key, value);
            write_stream(&mut stream, "Success\n");
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
            write_stream(&mut stream, &val);
        }
        Ok(Command::DELETE) => {
            let (key, _) = get_values_and_key(&string);
            if let Some(_) = map.get(&key) {
                match map.remove(&key) {
                    Some(_) => {
                        write_stream(&mut stream, "Key removed");
                    }
                    None => {
                        write_stream(&mut stream, "No Key found");
                    }
                }
            } else {
                write_stream(&mut stream, "No Key found");
            };
        }
        Err(e) => {
            write_stream(&mut stream, e.0.as_str());
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("connection failed: {e}");
                return;
            }
        };
        let mut buf = BufReader::new(&stream);
        let mut string = String::new();

        match buf.read_line(&mut string) {
            Ok(_) => {}
            Err(_) => {
                write_stream(&mut stream, "Unable to read the input");
                return;
            }
        };

        let string = string.trim().replace("\n", "");
        perform_commands(string, stream, &mut map);
    }
}

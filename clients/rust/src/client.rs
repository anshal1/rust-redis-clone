pub mod MyHashClient {
    use std::{
        io::{BufRead, BufReader, Error, Write},
        net::TcpStream,
    };

    pub struct Client {
        connection: TcpStream,
    }

    impl Client {
        pub fn new(host: &str, port: u32) -> Result<Self, Error> {
            let mut addr = String::new();
            addr.push_str(host);
            addr.push(':');
            addr.push_str(&port.to_string());
            let stream = TcpStream::connect(addr)?;
            Ok(Client { connection: stream })
        }
        // returns a error if any
        pub fn add(&mut self, key: &String, value: &String) -> Option<String> {
            let mut buf = BufReader::new(&self.connection);
            if let Err(e) = buf
                .get_mut()
                .write_all(format!("add {key} \"{value}\"\n").as_bytes())
            {
                println!("Unable to add the value");
                return Some(format!("Unable to add the value, {e}"));
            }
            let mut response = String::new();
            if let Err(e) = buf.read_line(&mut response) {
                println!("Unable to read the response from the server: {}", e);
                return None;
            } else {
                if response.replace("\n", "").is_empty() {
                    return None;
                }
                Some(response)
            }
        }
        // return the value, and returns None when any error occurs
        pub fn get(&mut self, key: &String) -> Option<String> {
            let mut buf = BufReader::new(&self.connection);
            if let Err(e) = buf.get_mut().write_all(format!("get {key}\n").as_bytes()) {
                println!("Unable to send the command to the server: {}", e);
                return None;
            } else {
                println!("Success");
            };
            let mut response = String::new();
            if let Err(e) = buf.read_line(&mut response) {
                println!("Unable to read the response from the server: {}", e);
                return None;
            }
            Some(response.replace('\n', ""))
        }
        // returns a error if any
        pub fn del(&mut self, key: &String) -> Option<String> {
            let mut buf = BufReader::new(&self.connection);
            if let Err(e) = buf.get_mut().write_all(format!("del {key}\n").as_bytes()) {
                println!("Unable to send the delete command to the server");
                return Some(format!("Unable to add the value, {e}"));
            }
            let mut response = String::new();
            if let Err(e) = buf.read_line(&mut response) {
                println!("Unable to read the response from the server: {}", e);
                return None;
            } else {
                if response.replace("\n", "").is_empty() {
                    return None;
                }
                Some(response)
            }
        }

        pub fn disconnect(&mut self) -> Result<String, String> {
            let mut buf = BufReader::new(&self.connection);
            // send end command
            if let Err(err) = buf.get_mut().write_all("end\n".as_bytes()) {
                println!("Unable to send the 'end' command");
                return Err(err.to_string());
            }
            if let Err(err) = self.connection.shutdown(std::net::Shutdown::Write) {
                println!("Unable to send shutdown request");
                return Err(err.to_string());
            };
            let mut goodbye = String::new();
            if let Err(err) = buf.read_line(&mut goodbye) {
                println!("Unable to read the goodbye message");
                return Err(err.to_string());
            } else {
                println!("{}", goodbye);
            };
            Ok("Connection Closed".to_string())
        }
    }
}

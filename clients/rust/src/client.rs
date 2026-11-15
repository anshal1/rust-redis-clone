pub mod MyHashClient {
    use std::{
        io::{BufRead, BufReader, Error, Write},
        net::{Shutdown, TcpStream},
    };

    pub struct Client {
        connection: Option<TcpStream>,
        host: String,
        port: u32,
    }

    fn connect(host: &str, port: u32) -> Result<TcpStream, Error> {
        let mut addr = String::new();
        addr.push_str(host);
        addr.push(':');
        addr.push_str(&port.to_string());
        let stream = TcpStream::connect(addr)?;
        return Ok(stream);
    }
    impl Client {
        pub fn new(host: &str, port: u32) -> Result<Self, Error> {
            let stream = connect(host, port)?;
            Ok(Client {
                connection: Some(stream),
                host: host.to_string(),
                port,
            })
        }
        fn check_and_create_new_connection(&mut self) -> Result<String, String> {
            if let Some(_) = &self.connection {
                return Ok("Connection exists".to_string());
            };
            let newconn = match connect(&self.host.as_str(), self.port) {
                Ok(connection) => connection,
                Err(_) => return Err("Unable to re-connect to the server".to_string()),
            };
            self.connection = Some(newconn);
            Ok("New connection created".to_string())
        }

        fn get_connection(&mut self) -> &TcpStream {
            match self.check_and_create_new_connection() {
                Ok(_) => {
                    if let Some(stream) = &self.connection {
                        &stream
                    } else {
                        panic!("Something went wrong while retreiving the connection")
                    }
                }
                Err(e) => panic!("{}", e),
            }
        }
        // returns a error string if any
        pub fn add(&mut self, key: &String, value: &String) -> Option<String> {
            let mut buf = BufReader::new(self.get_connection());
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
        // return the value, and returns "None" when no value is found and returs error string
        pub fn get(&mut self, key: &String) -> Result<String, String> {
            let mut buf = BufReader::new(self.get_connection());
            if let Err(e) = buf.get_mut().write_all(format!("get {key}\n").as_bytes()) {
                println!("Unable to send the command to the server: {}", e);
                return Err(format!("Unable to send the command to the server: {e}"));
            }
            let mut response = String::new();
            if let Err(e) = buf.read_line(&mut response) {
                println!("Unable to read the response from the server: {}", e);
                return Err(format!("Unable to read the response from the server: {e}"));
            }
            Ok(response.replace('\n', ""))
        }
        // returns a error if any
        pub fn del(&mut self, key: &String) -> Option<String> {
            let mut buf = BufReader::new(self.get_connection());
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
            let mut buf = BufReader::new(self.get_connection());
            // send end command
            if let Err(err) = buf.get_mut().write_all("end\n".as_bytes()) {
                println!("Unable to send the 'end' command");
                return Err(err.to_string());
            }
            if let Err(err) = buf.get_ref().shutdown(std::net::Shutdown::Write) {
                println!("Unable to shutdown the write from client to server");
                return Err(err.to_string());
            };
            let mut goodbye = String::new();
            if let Err(err) = buf.read_line(&mut goodbye) {
                println!("Unable to read the goodbye message");
                return Err(err.to_string());
            } else {
                println!("{}", goodbye);
            };
            self.connection = None;
            Ok("Connection Closed".to_string())
        }
    }
}

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;

pub fn get_url_header(listener: TcpListener) -> Result<String, Box<dyn Error>> {
    for stream in listener.incoming() {
        let mut stream = stream?;
        let buf_reader = BufReader::new(&mut stream);
        let request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let mut line = request[0].to_owned();
        line = line.split_off(6);
        line = line.strip_suffix(" HTTP/1.1").unwrap().to_string();
        return Ok(line);
    }

    Err("No response received".into())
}

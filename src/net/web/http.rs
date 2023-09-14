use httpstatus::StatusCode;
use std::{
    fmt::{
        Display,
        Error,
        Formatter
    },
    str::FromStr
};


pub enum HTTPMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
    NULL,
}

impl Display for HTTPMethod {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "{}",
            match self {
                HTTPMethod::CONNECT => "CONNECT",
                HTTPMethod::DELETE  => "DELETE",
                HTTPMethod::GET     => "GET",
                HTTPMethod::HEAD    => "HEAD",
                HTTPMethod::OPTIONS => "OPTIONS",
                HTTPMethod::PATCH   => "PATCH",
                HTTPMethod::POST    => "POST",
                HTTPMethod::PUT     => "PUT",
                HTTPMethod::TRACE   => "TRACE",
                HTTPMethod::NULL    => "",
            },
        )
    }
}

impl FromStr for HTTPMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        Ok(
            match s.trim() {
                "CONNECT" => HTTPMethod::CONNECT,
                "DELETE"  => HTTPMethod::DELETE,
                "GET"     => HTTPMethod::GET,
                "HEAD"    => HTTPMethod::HEAD,
                "OPTIONS" => HTTPMethod::OPTIONS,
                "PATCH"   => HTTPMethod::PATCH,
                "POST"    => HTTPMethod::POST,
                "PUT"     => HTTPMethod::PUT,
                "TRACE"   => HTTPMethod::TRACE,
                _         => HTTPMethod::NULL,
            }
        )
    }
}


pub struct Header(String, String);

impl Display for Header {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "{}: {}",
            self.0,
            self.1
        )
    }
}

impl FromStr for Header {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_index = match s.find(':') {
            Some(index) => index,
            None => return Err(Error),
        };

        Ok(
            Header {
                0: s[..split_index].trim().to_string(),
                1: s[split_index..].trim().to_string()
            }
        )
    }
}

pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub uri: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl HTTPRequest {
    pub fn format_headers(&self) -> String {
        let mut output: String = String::new();
        for header in &self.headers {
            output.push_str(&format!("{}: {}\r\n", header.0, header.1)[..]);
        }
        output
    }
}


impl Display for HTTPRequest {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "{} {} HTTP/1.1\r\n{}\r\n{}",
            self.method,
            self.uri,
            self.format_headers(),
            self.body,
        )
    }
}

impl FromStr for HTTPRequest {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut output: HTTPRequest = HTTPRequest { 
            method: HTTPMethod::NULL, 
            uri: String::from("/"), 
            headers: Vec::new(),
            body: String::new(), 
        };
        
        output.method = input
        .split(" ")
        .nth(0)
        .expect("Invalid input request string.")
        .parse()
        .expect("Invalid inptu request string.");

        output.uri = input
        .split(" ")
        .nth(1)
        .expect("Invalid input request string.")
        .to_string();

        // TODO: make this better, in this and in HTTPResponse.
        match input.find("\r\n") {
            Some(start_index) => {
                match input.find("\r\n\r\n") {
                    Some(end_index) => {
                        for header in input[start_index..end_index].lines() {
                            output.headers.push(header.parse().unwrap());
                        }
                        output.body = String::from(&input[end_index..]);
                    },
                    None => (),
                };
            },
            None => (),
        };

        Ok(
            output
        )
    }
}


// TODO: Test every thing about this.
pub struct HTTPResponse {
    pub status_code: StatusCode,
    pub headers: Vec<Header>,
    pub body: String
}


impl HTTPResponse {
    pub fn parse(response: String) -> Option<HTTPResponse> {
        let mut output: HTTPResponse = HTTPResponse { 
            status_code: StatusCode::Unknown(0), 
            headers: Vec::new(), 
            body: String::new(), 
        };
            
        

        output.status_code = match response[9..13].parse::<u16>() {
            Ok(code) => StatusCode::from(code),
            Err(_) => return None,
        };

        
        match response.find("\r\n") {
            Some(start_index) => {
                match response.find("\r\n\r\n") {
                    Some(end_index) => {
                        for header in response[start_index..end_index].lines() {
                            output.headers.push(header.parse().unwrap());
                        }
                        output.body = String::from(&response[end_index..]);
                    },
                    None => (),
                };
            },
            None => (),
        };

        Some(
            output
        )
    }

    pub fn format_headers(&self) -> String {
        let mut output: String = String::new();
        for header in &self.headers {
            output.push_str(&format!("{}: {}\r\n", header.0, header.1)[..]);
        }
        output
    }

    pub fn unparse(&self) -> String {
        format!("HTTP/1.1 {} \r\n{}\r\n{}", self.status_code, self.format_headers(), self.body)
    }
}

impl Display for HTTPResponse {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "HTTP/1.1 {} \r\n{}\r\n{}", 
            self.status_code, 
            self.format_headers(), 
            self.body
        )
    }
}



use std::{collections::HashMap, fs, io::Error};

pub struct Response {
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn html(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;

        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), contents.len().to_string());
        headers.insert("Content-Type".to_string(), "text/html".to_string());

        Ok(Self {
            headers,
            body: Some(contents),
        })
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response_str = String::from("HTTP/1.1 200 OK\r\n");

        for header in &self.headers {
            response_str.push_str(format!("{}: {}\r\n", header.0, header.1).as_str());
        }
        response_str.push_str("\r\n");

        if let Some(body) = &self.body {
            response_str.push_str(body.as_str());
        }

        response_str
    }
}

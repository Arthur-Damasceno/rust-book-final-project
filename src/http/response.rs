use {
    super::status::Status,
    async_std::{fs, io::Error},
    std::collections::HashMap,
};

pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub async fn html(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename).await?;

        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), contents.len().to_string());
        headers.insert("Content-Type".to_string(), "text/html".to_string());

        Ok(Self {
            status: Status::Ok,
            headers,
            body: Some(contents),
        })
    }

    pub fn status(&mut self, status: Status) {
        self.status = status
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response_str = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.code(),
            self.status.reason_phrase()
        );

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

#[cfg(test)]
mod tests {
    use {
        super::*,
        async_std::{
            fs::{remove_file, File},
            prelude::*,
        },
    };

    async fn create_file_with_contents(path: &str, contents: &str) {
        let mut file = File::create(path).await.unwrap();
        file.write_all(contents.as_bytes()).await.unwrap();
    }

    #[async_std::test]
    async fn should_read_file_and_return_response() {
        let filename = "test.html";
        let contents = "<!DOCTYPE html>\n<html>\n</html>";

        create_file_with_contents(filename, contents).await;
        let response = Response::html(filename).await.unwrap();
        remove_file(filename).await.unwrap();
        let result = response.to_string();

        assert!(result.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(result.ends_with(&format!("\r\n{}", contents)));
        assert!(result.contains("Content-Type: text/html\r\n"));
        assert!(result.contains(&format!("Content-Length: {}\r\n", contents.len())));
    }
}

pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
        }
    }

    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
        }
    }
}

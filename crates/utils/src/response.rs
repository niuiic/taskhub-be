use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Code {
    OK = 200,
    Err = 500,
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    code: Code,
    msg: Option<String>,
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn from_result<E: Into<String>>(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => Response {
                code: Code::OK,
                msg: None,
                data: Some(value),
            },
            Err(err) => Response {
                code: Code::Err,
                msg: Some(err.into()),
                data: None,
            },
        }
    }
}

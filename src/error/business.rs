use super::base::Error;

pub const BIZ_INTERNAL_ERROR: i16 = -1;

#[derive(Debug)]
pub struct BizError {
    code: i16,
    message: String,
}

pub type BizResult<T> = std::result::Result<T, BizError>;

impl BizError {
    pub fn new(code: i16, message: String) -> Self {
        Self { code, message }
    }

    pub fn code(&self) -> i16 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<Error> for BizError {
    fn from(value: Error) -> Self {
        Self {
            code: BIZ_INTERNAL_ERROR,
            message: value.to_string(),
        }
    }
}

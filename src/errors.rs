use std::fmt;
use bcrypt::BcryptError;
use diesel::result;


pub enum MyError {
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String)
}

impl From<BcryptError> for MyError {
    fn from(error: BcryptError) -> Self {
        MyError::HashError(error)
    }
}



impl From<result::Error> for MyError {
    fn from(error: result::Error) -> Self {
        MyError::DBError(error)
    }
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::HashError(error) => write!(f, "{}", error),
            MyError::DBError(error) => write!(f, "{}", error),
            MyError::PasswordNotMatch(error) => write!(f, "{}", error),
            MyError::WrongPassword(error) => write!(f, "{}", error)
        }
    }
}
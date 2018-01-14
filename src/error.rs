use std::error::Error;
use std::fmt;
use std;


#[derive(Debug)]
pub struct ObjError(pub String);

pub type Result<T> = std::result::Result<T, ObjError>;



impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &ObjError(ref s) = self;
        write!(f, "{}", s)
    }
}

impl Error for ObjError {
    fn description(&self) -> &str {
        let &ObjError(ref s) = self;
        return s.as_str();
    }
}

//impl From<NoneError> for ObjError {
//    fn from(err : NoneError) -> ObjError {
//      ObjError(format!("{:?}", err))
//    }
//}

impl From<std::num::ParseFloatError> for ObjError {
    fn from(err : std::num::ParseFloatError) -> ObjError {
        ObjError(format!("{:?}", err))
    }
}

impl From<std::num::ParseIntError> for ObjError {
    fn from(err : std::num::ParseIntError) -> ObjError {
        ObjError(format!("{:?}", err))
    }
}

impl From<std::string::ParseError> for ObjError {
    fn from(err : std::string::ParseError) -> ObjError {
        ObjError(format!("{:?}", err))
    }
}

impl From<String> for ObjError {
    fn from(err : String) -> ObjError {
        ObjError(format!("{:?}", err))
    }
}

impl<'a> From<&'a str> for ObjError {
    fn from(err : &'a str) -> ObjError {
        ObjError(format!("{}", err))
    }
}


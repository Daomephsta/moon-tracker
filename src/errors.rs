//Why in tarnation is error handling such a PITA in Rust? It's supposed to be better.
use azul::error::Error as AzulError;
use simple_error::SimpleError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InternalError
{
    Azul(AzulError),
    Simple(SimpleError)
}

impl Error for InternalError {}

impl fmt::Display for InternalError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        return match self
        {
            InternalError::Azul(e) => write!(f, "{}", e),
            InternalError::Simple(e) => write!(f, "{}", e)
        }
    }
}

impl From<AzulError> for InternalError
{
    fn from(error: AzulError) -> Self 
    {
        InternalError::Azul(error)
    }
}

impl From<SimpleError> for InternalError
{
    fn from(error: SimpleError) -> Self 
    {
        InternalError::Simple(error)
    }
}
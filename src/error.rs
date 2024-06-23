/// Error type that implements `std::error::Error` to easily return those kind
/// of errors arbitrarily in the code.
#[derive(Debug)]
pub struct StdError(pub String);

impl std::fmt::Display for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard error: {}", self.0)
    }
}

impl std::error::Error for StdError {}

impl StdError {
    pub fn to_boxed(&self) -> Box<Self> {
        Box::new(Self(self.0.clone()))
    }

    pub fn to_boxed_err(&self) -> Result<(), Box<dyn std::error::Error>> {
        Err(self.to_boxed())
    } 
}

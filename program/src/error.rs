use pinocchio::program_error::ProgramError;

/// Errors that may be returned by myproject.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MyProjectError {
    #[error("Invalid")]
    Invalid,
}

impl From<MyProjectError> for ProgramError {
    fn from(e: MyProjectError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

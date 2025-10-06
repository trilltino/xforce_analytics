use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Password too short (minimum {0} characters)")]
    PasswordTooShort(usize),
    #[error("Field '{0}' is required")]
    RequiredField(String),
    #[error("{0}")]
    Custom(String),
}

pub type ValidationResult<T> = Result<T, ValidationError>;

pub fn validate_email(email: &str) -> ValidationResult<()> {
    if email.is_empty() {
        return Err(ValidationError::RequiredField("email".to_string()));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(ValidationError::InvalidEmail);
    }

    Ok(())
}

pub fn validate_password(password: &str, min_length: usize) -> ValidationResult<()> {
    if password.is_empty() {
        return Err(ValidationError::RequiredField("password".to_string()));
    }

    if password.len() < min_length {
        return Err(ValidationError::PasswordTooShort(min_length));
    }

    Ok(())
}

pub fn validate_required(field_name: &str, value: &Option<String>) -> ValidationResult<()> {
    if value.is_none() || value.as_ref().unwrap().is_empty() {
        return Err(ValidationError::RequiredField(field_name.to_string()));
    }
    Ok(())
}

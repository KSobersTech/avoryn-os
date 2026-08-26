use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,

    #[serde(skip)]
    pub context: Option<String>,
}

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::new("VALIDATION_ERROR", message)
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        let resource = resource.into();

        Self::new("NOT_FOUND", format!("{resource} was not found."))
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new("CONFLICT", message)
    }

    pub fn database(context: impl Into<String>) -> Self {
        Self::new(
            "DATABASE_ERROR",
            "AVORYN could not complete the database operation.",
        )
        .with_context(context)
    }

    pub fn internal(context: impl Into<String>) -> Self {
        Self::new("INTERNAL_ERROR", "AVORYN encountered an unexpected error.").with_context(context)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for AppError {}

//! # Compiler errors.

use super::BuilderError;
use cairo_lang_sierra::{
    edit_state::EditStateError,
    ids::{ConcreteLibfuncId, ConcreteTypeId},
    program_registry::ProgramRegistryError,
};
use std::{fmt, ops::Deref};
use thiserror::Error;

/// A [`Result`](std::result::Result) alias with the error type fixed to [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Wrapper for the error type and the error's origin backtrace (soon™).
#[derive(Error)]
pub struct Error {
    // TODO: Enable once it stabilizes.
    // pub backtrace: Backtrace,
    /// The actual error.
    pub source: ErrorImpl,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.source, f)
    }
}

impl Deref for Error {
    type Target = ErrorImpl;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<E> From<E> for Error
where
    ErrorImpl: From<E>,
{
    fn from(error: E) -> Self {
        Self {
            // backtrace: Backtrace::capture(),
            source: error.into(),
        }
    }
}

impl<E> From<E> for Box<Error>
where
    ErrorImpl: From<E>,
{
    fn from(error: E) -> Self {
        Self::new(Error::from(error))
    }
}

// Manual implementation necessary because `#[derive(Debug)]` requires that `TType` and `TLibfunc`
// both implement `Debug`, which isn't the case.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            // .field("backtrace", &self.backtrace)
            .field("source", &self.source)
            .finish()
    }
}

/// A compilation error.
#[derive(Error)]
pub enum ErrorImpl {
    /// Failed to edit the Sierra state. This should mean an invalid Sierra has been provided to the
    /// compiler.
    #[error(transparent)]
    EditStateError(#[from] EditStateError),
    /// An MLIR error has occurred.
    #[error(transparent)]
    MlirError(#[from] melior::Error),
    /// The program registry returned an error. This should mean an invalid Sierra has been provided
    /// to the compiler.
    #[error(transparent)]
    ProgramRegistryError(#[from] Box<ProgramRegistryError>),

    /// A [TypeBuilder](crate::types::TypeBuilder) error.
    #[error("Error building type '{type_id}': {error}")]
    TypeBuilderError {
        /// The type which caused an error.
        type_id: ConcreteTypeId,
        /// The actual error.
        error: BuilderError,
    },
    /// A [LibfuncBuilder](crate::libfuncs::LibfuncBuilder) error.
    #[error("Error building type '{type_id}': {error}")]
    LibfuncBuilderError {
        /// The type which caused an error.
        type_id: ConcreteLibfuncId,
        /// The actual error.
        error: BuilderError,
    },
}

// Manual implementation necessary because `#[derive(Debug)]` requires that `TType` and `TLibfunc`
// both implement `Debug`, which isn't the case.
impl fmt::Debug for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EditStateError(arg0) => f.debug_tuple("EditStateError").field(arg0).finish(),
            Self::MlirError(arg0) => f.debug_tuple("MlirError").field(arg0).finish(),
            Self::ProgramRegistryError(arg0) => {
                f.debug_tuple("ProgramRegistryError").field(arg0).finish()
            }
            Self::TypeBuilderError { type_id, error } => f
                .debug_struct("TypeBuilderError")
                .field("type_id", type_id)
                .field("error", error)
                .finish(),
            Self::LibfuncBuilderError { type_id, error } => f
                .debug_struct("LibfuncBuilderError")
                .field("type_id", type_id)
                .field("error", error)
                .finish(),
        }
    }
}

pub(crate) fn make_type_builder_error(
    id: &ConcreteTypeId,
) -> impl '_ + FnOnce(BuilderError) -> Error
where
{
    move |source| {
        ErrorImpl::TypeBuilderError {
            type_id: id.clone(),
            error: source,
        }
        .into()
    }
}

pub(crate) fn make_libfunc_builder_error(
    id: &ConcreteLibfuncId,
) -> impl '_ + FnOnce(BuilderError) -> Error
where
{
    move |source| {
        ErrorImpl::LibfuncBuilderError {
            type_id: id.clone(),
            error: source,
        }
        .into()
    }
}

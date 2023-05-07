//! All sorts of conversions between error types defined by the crate.
use super::*;

impl From<UserError> for Err {
    fn from(value: UserError) -> Self {
        Self::UserError(value)
    }
}

impl From<ServerError> for Err {
    fn from(value: ServerError) -> Self {
        Self::ServerError(value)
    }
}

impl<T> From<T> for ApiError
where
    T: ApiErrorTrait,
{
    fn from(value: T) -> Self {
        Self {
            status: value.status(),
            msg: value.msg(),
            links: value.links().to_owned(),
        }
    }
}

use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: Cow<'static, str>,
}

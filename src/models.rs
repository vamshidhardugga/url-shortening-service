use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize)]
pub struct Response {
    pub message: Cow<'static, str>,
}

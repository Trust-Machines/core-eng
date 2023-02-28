use serde::Serialize;

#[derive(Serialize)]
pub struct DispatchCommand<T>(pub String, pub T);

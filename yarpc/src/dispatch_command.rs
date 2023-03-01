use serde::Serialize;

/// See `DispatchCommand` in `lib.ts`.
#[derive(Serialize)]
pub struct DispatchCommand<T>(pub String, pub T);

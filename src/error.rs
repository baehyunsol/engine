use crate::engine::EngineType;

#[derive(Debug)]
pub enum Error {
    PathError(String),
    RenderError(EngineType, String)
}
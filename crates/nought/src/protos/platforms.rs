use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub enum Platforms {
    MacOS, Windows
}
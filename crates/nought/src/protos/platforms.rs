use serde::Deserialize;

#[derive(Deserialize)]
pub enum Platforms {
    MacOS, Windows
}
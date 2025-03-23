#![allow(clippy::derivable_impls)] // better to manually impl for clearer intent

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub fs: Filesystem,
    pub graphics: Graphics,
    pub behaviour: Behaviour,
}

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Filesystem {
    pub game_dir: String,
}

impl Default for Filesystem {
    fn default() -> Self {
        Self {
            game_dir: ".".into(),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Graphics {
    pub force_downlevel: bool,
    pub vsync: bool,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            force_downlevel: false,
            vsync: true,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Behaviour {
    pub abort_on_panic: bool,
}
impl Default for Behaviour {
    fn default() -> Self {
        Self {
            abort_on_panic: false,
        }
    }
}

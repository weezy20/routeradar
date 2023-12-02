use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum Mode {
    Next,
    Svelte,
}

#[derive(Serialize, Deserialize)]
struct Route {
    path: String,
    param: Option<Vec<Route>>,
}

#[derive(Serialize, Deserialize)]
pub struct RR {
    mode: Mode,
    routes: Vec<Route>,
}

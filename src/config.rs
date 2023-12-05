/// Config code baby
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the mode of operation.
#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, JsonSchema)]
pub enum Mode {
    /// For Nextjs mode.
    Next,
    /// For Sveltekit mode.
    Svelte,
}

/// Represents a route with its properties.
#[derive(Serialize, Deserialize, JsonSchema)]
struct Route {
    /// The path of the route.
    path: String,
    /// Indicates if the route is dynamic.
    dynamic: Option<bool>,
    /// Indicates if the route is a catch-all.
    catch_all: Option<bool>,
    /// Children routes of the current route.
    children: Option<Vec<Route>>,
}

/// Represents a routing configuration.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RR {
    /// The mode of operation.
    mode: Mode,
    /// The routes of the application.
    routes: Vec<Route>,
}

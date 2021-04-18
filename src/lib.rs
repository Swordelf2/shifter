/*
mod loading;
mod menu;
mod game;
mod state;
*/

mod asset;
mod config;
mod loading;
// mod menu
mod game;
mod run;
mod state;

#[cfg(feature = "debug")]
mod debug;

pub use run::run;

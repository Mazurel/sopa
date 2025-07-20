use gloo::history::{BrowserHistory, History};
use serde::Serialize;

pub type Path = String;

// TODO: Allow to add query parameters and handle them somewhere.
//       use: `gloo::history::query` - https://docs.rs/gloo/latest/gloo/history/query/index.html
#[derive(Serialize)]
#[allow(unused)]
pub struct Query {}

fn get_history() -> BrowserHistory {
    gloo::history::BrowserHistory::new()
}

pub fn read_window_path() -> Path {
    get_history().location().path().to_string()
}

pub fn set_window_path(path: String) {
    get_history().push(path);
}

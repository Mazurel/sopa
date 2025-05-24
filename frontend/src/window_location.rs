use web_sys::window;

pub fn read_window_path() -> Option<String> {
    let window = window()?;
    match window.location().pathname() {
        Err(_) => None,
        Ok(pathname) => Some(pathname),
    }
}

#[allow(unused)]
pub fn set_window_path(path: String) {
    unimplemented!("Use gloo history API here")
}

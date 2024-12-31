use app::App;

mod app;
mod navigation;
mod routes;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
// Init rust-i18n, refer to lib readme
i18n!("locales");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    rust_i18n::set_locale("pl");
    yew::Renderer::<App>::new().render();
}

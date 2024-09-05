pub mod mods;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    unsafe { std::env::set_var("RUST_LOG", "info") };
    pretty_env_logger::init();

    let a_mod = mods::Mod::new(String::from("mods/test_mod/"));
    info!("Mod name: {}", a_mod.debug_name());
    a_mod.init();
}

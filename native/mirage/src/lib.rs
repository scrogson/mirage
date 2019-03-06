use rustler::schedule::SchedulerFlags::*;

mod atoms;
mod mirage;

fn load(env: rustler::Env, _info: rustler::Term) -> bool {
    mirage::load(env);
    true
}

rustler::rustler_export_nifs! {
    "Elixir.Mirage",
    [
        ("from_bytes", 1, mirage::from_bytes, DirtyIo),
        ("resize", 3, mirage::resize, DirtyCpu),
    ],
    Some(load)
}

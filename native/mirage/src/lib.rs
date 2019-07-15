use rustler::{Env, SchedulerFlags::*, Term};

mod atoms;
mod mirage;

fn load(env: Env, _info: Term) -> bool {
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

use rustler::{SchedulerFlags::*, Term};

mod atoms;
mod mirage;

rustler::rustler_export_nifs! {
    "Elixir.Mirage",
    [
        ("from_bytes", 1, mirage::from_bytes, DirtyIo),
        ("resize", 3, mirage::resize, DirtyCpu),
    ],
    Some(mirage::load)
}

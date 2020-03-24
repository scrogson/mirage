mod atoms;
mod mirage;

rustler::init! {
    "Elixir.Mirage.Native",
    [
        mirage::from_bytes,
        mirage::resize
    ],
    load = mirage::load
}

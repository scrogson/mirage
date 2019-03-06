# Mirage

> Image manipulation for Elixir.

This library provides a [Rust] implemented [NIF] which currently supports
resizing images.

## Installation

Because this library is partially implemented in [Rust], you will need to have
the Rust toolchain installed on your system.

### Install Rust

```
curl https://sh.rustup.rs -sSf | sh
```

### Add package to your mix.exs

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `mirage` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:mirage, "~> 0.1.0"}
  ]
end
```

## Basic usage

```ex
{:ok, bytes} = File.read("/path/to/image.png")
{:ok, mirage} = Mirage.from_bytes(bytes)
{:ok, mirage} = Mirage.resize(mirage.resource, 400, 300)

mirage.width #=> 400
mirage.height #=> 300

File.write!("/path/to/resized-400x300.png", mirage.bytes)
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/mirage](https://hexdocs.pm/mirage).

[Rust]: https://www.rust-lang.org/
[NIF]: http://erlang.org/doc/man/erl_nif.html

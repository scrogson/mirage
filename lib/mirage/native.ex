defmodule Mirage.Native do
  use Rustler, otp_app: :mirage

  def from_bytes(_path), do: :erlang.nif_error(:nif_not_loaded)
  def resize(_resource, _width, _height), do: :erlang.nif_error(:nif_not_loaded)
end

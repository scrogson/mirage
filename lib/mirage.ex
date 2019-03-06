defmodule Mirage do
  use Rustler, otp_app: :mirage

  defstruct bytes: nil,
            byte_size: nil,
            extension: nil,
            height: nil,
            width: nil,
            resource: nil

  def from_bytes(_path), do: :erlang.nif_error(:nif_not_loaded)
  def resize(_resource, _width, _height), do: :erlang.nif_error(:nif_not_loaded)
end

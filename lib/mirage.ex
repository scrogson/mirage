defmodule Mirage do
  defstruct byte_size: nil,
            format: nil,
            height: nil,
            width: nil,
            resource: nil

  def from_bytes(bytes) do
    Mirage.Native.from_bytes(bytes)
  end

  def resize(%Mirage{} = mirage, width, height) do
    Mirage.Native.resize(mirage.resource, width, height)
  end
end

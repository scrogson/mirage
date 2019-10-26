defmodule MirageTest do
  use ExUnit.Case

  doctest Mirage

  setup do
    {:ok, bytes} = File.read("./test/support/images/scrogson.jpeg")

    {:ok, bytes: bytes}
  end

  test "from_bytes", %{bytes: bytes} do
    byte_size = byte_size(bytes)

    {:ok, mirage} = Mirage.from_bytes(bytes)

    assert mirage.byte_size == byte_size
    assert mirage.format == :jpg
    assert mirage.width == 460
    assert mirage.height == 460
    assert is_reference(mirage.resource)
  end

  test "resize", %{bytes: bytes} do
    {:ok, mirage} = Mirage.from_bytes(bytes)

    byte_size = byte_size(bytes)

    {:ok, _new_bytes, mirage} = Mirage.resize(mirage, 200, 200)

    assert mirage.byte_size > byte_size
    assert mirage.width == 200
    assert mirage.height == 200
  end
end

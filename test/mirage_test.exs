defmodule MirageTest do
  use ExUnit.Case
  doctest Mirage

  test "greets the world" do
    assert Mirage.hello() == :world
  end
end

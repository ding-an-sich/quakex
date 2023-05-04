defmodule QuakexTest do
  use ExUnit.Case
  doctest Quakex

  test "greets the world" do
    assert Quakex.hello() == :world
  end
end

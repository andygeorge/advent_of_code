defmodule Day17Test do
  use ExUnit.Case

  test "it counts active cells" do
    assert Day17.part_one(InputTestFile) == 112
  end
end

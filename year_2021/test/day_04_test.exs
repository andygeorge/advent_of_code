defmodule Day04Test do
  use ExUnit.Case, async: true

  test "it computes the score of the winning board" do
    assert Day04.part_one(InputTestFile) == 4512
  end
end

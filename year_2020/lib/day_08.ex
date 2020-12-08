defmodule Day08 do
  def part_one(file_reader \\ InputFile) do
    file_reader.contents_of(8, :stream)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&parse/1)
    |> Day08.TapeMachine.new()
    |> Day08.TapeMachine.detect_loop()
  end

  defp parse(<<opcode :: binary-size(3), " +", value :: binary>>), do: {opcode, String.to_integer(value)}
  defp parse(<<opcode :: binary-size(3), " -", value :: binary>>), do: {opcode, -String.to_integer(value)}
end

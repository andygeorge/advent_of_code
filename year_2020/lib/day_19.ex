defmodule Day19 do
  def part_one(file_reader \\ InputFile) do
    {rule_strs, ["" | strs]} = file_reader.contents_of(19, :stream)
    |> Enum.map(&String.trim/1)
    |> Enum.split_while(fn
      "" -> false
      _ -> true
    end)

    rule = build(rule_strs)
    Enum.count(strs, &(rule.(&1)))
  end

  def build(rule_strs) do
    f = rule_strs
    |> Enum.map(&parse/1)
    |> build_pass([], %{})
    |> Map.get(0)

    fn s -> f.(s) == true end
  end

  def parse(rule_str) do
    [n, deps] = String.split(rule_str, ": ")

    {String.to_integer(n), parse_deps(deps)}
  end

  def parse_deps(<<"\"", char :: binary-size(1), "\"">>), do: char
  def parse_deps(str) do
    str
    |> String.split(" | ")
    |> Enum.map(fn l -> String.split(l) |> Enum.map(&String.to_integer/1) end)
  end

  def build_pass([], [], map), do: map
  def build_pass([], pending, map), do: build_pass(pending, [], map)
  def build_pass([{index, char} | rest], pending, map) when is_binary(char) do
    build_pass(rest, pending, Map.put(map, index, Day19.Rule.build(char)))
  end
  def build_pass([{index, rule_numbers} = rule | rest], pending, map) do
    case rule_lookup(rule_numbers, map) do
      :error -> build_pass(rest, [rule | pending], map)
      translated -> build_pass(rest, pending, Map.put(map, index, Day19.Rule.build(translated)))
    end
  end

  def rule_lookup(rule_numbers, map), do: rule_lookup(rule_numbers, map, [])
  def rule_lookup([], _map, rules), do: Enum.reverse(rules)
  def rule_lookup([n | rest], map, rules) when is_list(n) do
    case rule_lookup(n, map) do
      :error -> :error
      translated -> rule_lookup(rest, map, [translated | rules])
    end
  end
  def rule_lookup([n | rest], map, rules) do
    case Map.fetch(map, n) do
      :error -> :error
      {:ok, rule} -> rule_lookup(rest, map, [rule | rules])
    end
  end
end

defmodule AoC2 do
  defmodule Part1 do
    @moduledoc false
    def move("forward", step, state = %{horizontal: h}) do
      %{state | horizontal: h + step}
    end

    def move("down", step, state = %{depth: d}) do
      %{state | depth: d + step}
    end

    def move("up", step, state = %{depth: d}) do
      %{state | depth: d - step}
    end
  end

  defmodule Part2 do
    @moduledoc false
    def move("forward", step, state = %{horizontal: h, depth: d, aim: a}) do
      %{state | horizontal: h + step, depth: d + a * step}
    end

    def move("down", step, state = %{aim: a}) do
      %{state | aim: a + step}
    end

    def move("up", step, state = %{aim: a}) do
      %{state | aim: a - step}
    end
  end

  @doc """
  Solves the problem.

  Parameters:
  - `filename`: path to the file with the commands.
  """
  def solve(filename \\ "2.txt") do
    lines =
      File.read!(filename)
      |> String.split("\n")
      |> Stream.reject(&(&1 == ""))

    Enum.each([Part1, Part2], fn module ->
      lines
      |> Enum.reduce(%{depth: 0, horizontal: 0, aim: 0}, fn line, acc ->
        [command, step] = String.split(line, " ")

        module.move(command, String.to_integer(step), acc)
      end)
      |> then(fn %{depth: d, horizontal: h} -> d * h end)
      |> IO.puts()
    end)
  end
end

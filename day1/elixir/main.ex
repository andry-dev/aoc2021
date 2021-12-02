defmodule AoC1 do
  defp calculate(depths, window_width \\ 1) do
    depths
    |> Stream.chunk_every(2, window_width, :discard)
    |> Enum.reduce(0, fn [prev, curr], acc ->
      if curr > prev do
        acc + 1
      else
        acc
      end
    end)
  end

  def solve() do
    depths =
      File.read!("1.txt")
      |> String.split("\n")
      |> Stream.reject(&(&1 == ""))
      |> Stream.map(&String.to_integer(&1))

    IO.puts(calculate(depths))

    depths
    |> Stream.chunk_every(3, 1)
    |> Stream.map(&Enum.sum(&1))
    |> calculate()
    |> IO.puts()
  end
end

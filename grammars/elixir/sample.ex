defmodule Sample.Shape do
  @moduledoc """
  A sample.
  """

  defstruct [:kind, radius: 0.0, width: 0.0, height: 0.0]

  @type t :: %__MODULE__{kind: atom(), radius: float()}
  @limit 10

  @spec area(t()) :: float()
  def area(%__MODULE__{kind: :circle, radius: r}), do: :math.pi() * r * r
  def area(%__MODULE__{kind: :rect, width: w, height: h}) when w > 0, do: w * h
  def area(_), do: 0.0

  def largest([]), do: nil
  def largest(items), do: Enum.max(items)

  defp describe(shape) do
    case shape.kind do
      :circle -> "a circle of #{shape.radius}"
      other -> "a #{other}"
    end
  end

  def run do
    shapes = [%__MODULE__{kind: :circle, radius: 1.0}, %__MODULE__{kind: :rect, width: 2.0, height: 3.0}]

    # A comment.
    shapes
    |> Enum.map(&area/1)
    |> Enum.filter(fn a -> a < @limit end)
    |> Enum.each(&IO.puts/1)

    with {:ok, n} <- parse("42") do
      shapes |> List.first() |> describe() |> IO.puts()
      IO.puts(n)
    else
      :error -> raise ArgumentError, "not a number"
    end
  end

  defp parse(text) do
    case Integer.parse(text) do
      {n, ""} -> {:ok, n}
      _ -> :error
    end
  end
end

defmodule Adventofcode.DayOne do
  def finalfloor("") do
    0
  end

  def finalfloor("\n" <> str) do
    finalfloor(str)
  end

  def finalfloor("(" <> str) do
    1 + finalfloor(str)
  end

  def finalfloor(")" <> str) do
    -1 + finalfloor(str)
  end

  def _basementat(start, "") do
    case finalfloor(start) do
      -1 -> 1
    end
  end

  def _basementat(start, finito) do
    if finalfloor(start) == -1 do
      1
    else
      base = byte_size(")")
      <<x :: binary-size(base), rest :: binary>> = finito
      1 + _basementat(start <> x, rest)
    end
  end

  def basementat(full) do
    base = byte_size(")")
    <<x :: binary-size(base), rest :: binary>> = full
    _basementat(x, rest)
  end
end

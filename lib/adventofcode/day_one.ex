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
end

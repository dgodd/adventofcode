defmodule Adventofcode.DayTwoTest do
  use ExUnit.Case
  require Adventofcode.DayTwo

  test "A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet." do
    assert Adventofcode.DayTwo.squarefeet(2,3,4) == 58
    assert Adventofcode.DayTwo.squarefeet("2x3x4") == 58
  end

  test "A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet." do
    assert Adventofcode.DayTwo.squarefeet(1,1,10) == 43
    assert Adventofcode.DayTwo.squarefeet("1x1x10") == 43
  end

  test "input file for squarefeet" do
    num = File.read!("test/day_two.txt")
    |> String.split
    |> Enum.map(&(Adventofcode.DayTwo.squarefeet &1))
    |> Enum.reduce(0, fn(x, acc) -> x + acc end)
    assert num == 1598415
  end

  test "A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet." do
    assert Adventofcode.DayTwo.lengthribbon(2,3,4) == 34
    assert Adventofcode.DayTwo.lengthribbon("2x3x4") == 34
  end

  test "A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet." do
    assert Adventofcode.DayTwo.lengthribbon(1,1,10) == 14
    assert Adventofcode.DayTwo.lengthribbon("1x1x10") == 14
  end

  test "input file for lengthribbon" do
    num = File.read!("test/day_two.txt")
    |> String.split
    |> Enum.map(&(Adventofcode.DayTwo.lengthribbon &1))
    |> Enum.reduce(0, fn(x, acc) -> x + acc end)
    assert num == 3812909
  end
 end

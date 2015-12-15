defmodule Adventofcode.DayOneTest do
  use ExUnit.Case
  require Adventofcode.DayOne

  test "(()) and ()() both result in floor 0" do
    assert Adventofcode.DayOne.finalfloor("(())") == 0
    assert Adventofcode.DayOne.finalfloor("()()") == 0
  end

  test "((( and (()(()( both result in floor 3" do
    assert Adventofcode.DayOne.finalfloor("(((") == 3
    assert Adventofcode.DayOne.finalfloor("(()(()(") == 3
  end

  test "))((((( also results in floor 3" do
    assert Adventofcode.DayOne.finalfloor("))(((((") == 3
  end

  test "()) and ))( both result in floor -1 (the first basement level)." do
    assert Adventofcode.DayOne.finalfloor("())") == -1
    assert Adventofcode.DayOne.finalfloor("))(") == -1

  end

  test "))) and )())()) both result in floor -3" do
    assert Adventofcode.DayOne.finalfloor(")))") == -3
    assert Adventofcode.DayOne.finalfloor(")())())") == -3
  end

  test "input file" do
    {:ok, str} = File.read "test/day_one.txt"
    assert Adventofcode.DayOne.finalfloor(str) == 232
  end
end

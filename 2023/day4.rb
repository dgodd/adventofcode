require 'minitest/autorun'

class Day4 < Minitest::Test
  def parse(filename)
    lines = File.read(filename).split("\n")
    lines.map do |l|
      m = l.match(/Card\s+(\d+):\s+(.*) \|\s+(.*)/)
      num = m[1].to_i
      winning = m[2].split(/\s+/).map(&:to_i)
      actual = m[3].split(/\s+/).map(&:to_i)
      [num, winning, actual]
    end
  end

  def part1(filename)
    data = parse(filename)
    data.map do |l|
      reps = l[1] & l[2]
      num = reps.length == 0 ? 0 : 2 ** (reps.length - 1)
      num
    end.sum
  end

  def test_sample1
    num = part1('fixtures/day4/sample1.txt')
    assert_equal 13, num
  end

  def test_part1
    num = part1('fixtures/day4/input.txt')
    assert_equal num, 21105
  end
end

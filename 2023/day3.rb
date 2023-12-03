require 'minitest/autorun'

class Day4 < Minitest::Test
  def parse(filename)
    lines = File.read(filename).split("\n")
    lines
  end

  def part1(filename)
    lines = parse(filename)
    # found.sum
  end

  def test_sample1
    num = part1('fixtures/day4/sample1.txt')
    assert_equal num, 4361
  end

  # def test_part1
  #   num = part1('fixtures/day4/input.txt')
  #   assert_equal num, 549908
  # end
end

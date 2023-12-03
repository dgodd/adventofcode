require 'minitest/autorun'

class Day1 < Minitest::Test
  def day1(filename)
    txt = File.read(filename)
    lines = txt.split("\n").map { |l| l.split('').grep(/\d/) }
    actual = lines.map { |l| (l.first + l.last).to_i }.sum
  end

  def test_sample1
    num = day1('fixtures/day1/sample1.txt')
    assert_equal num, 142
  end

  def test_day1
    num = day1('fixtures/day1/input1.txt')
    assert_equal num, 55607
  end
end

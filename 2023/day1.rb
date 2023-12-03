require 'minitest/autorun'

NUMBERS = %w{one two three four five six seven eight nine}.freeze

class Day1 < Minitest::Test
  def part1(filename)
    txt = File.read(filename)
    lines = txt.split("\n")
    lines = lines.map { _1.split('').grep(/\d/) }
    lines.map { (_1.first + _1.last).to_i }.sum
  end

  def test_sample1
    num = part1('fixtures/day1/sample1.txt')
    assert_equal num, 142
  end

  def test_part1
    num = part1('fixtures/day1/input1.txt')
    assert_equal num, 55607
  end

  def part2(filename)
    txt = File.read(filename)
    lines = txt.split("\n").map do |l|
      l2 = "#{l}"
      arr = []
      while l.length > 0
        if l.match(/^\d/)
          arr << l[0].to_i
        else
          NUMBERS.each.with_index do |w,idx|
            if l.start_with?(w)
              arr << idx + 1
              break
            end
          end
        end
        l = l[1..]
      end
      # p [arr, l2, (arr.first * 10) + arr.last]
      arr
    end
    lines.map { (_1.first * 10) + _1.last }.sum
  end

  def test_sample2
    num = part2('fixtures/day1/sample2.txt')
    assert_equal num, 281
  end

  def test_part2
    num = part2('fixtures/day1/input1.txt')
    assert_equal num, 55291
  end
end

require 'minitest/autorun'
require 'minitest/focus'

class Day9 < Minitest::Test
  def parse(filename)
    File.read(filename).split(/\n/).map { _1.split(/\s+/).map(&:to_i) }
  end

  def next_val(values)
    # puts "RCVD: #{values.inspect}"
    return 0 if values.all?(0)

    diffs = values.each_cons(2).map { |(a, b)| b - a }

    # p diffs
    nv = next_val(diffs)
    # puts "NV: #{nv}"

    # puts "NV2: #{values.last + nv} -- #{values.last} + #{nv}"
    values.last + nv
  end

  def test_sample_1
    assert_equal 18, next_val(%w{0 3 6 9 12 15}.map(&:to_i))
  end

  def test_sample_1
    assert_equal 28, next_val(%w{1 3 6 10 15 21}.map(&:to_i))
  end

  def test_sample_1
    assert_equal 68, next_val(%w{10 13 16 21 30 45}.map(&:to_i))
  end

  def part1(filename)
    values = parse(filename)
    values.sum do |vs|
      next_val(vs)
    end
  end

  focus def test_part1
    num = part1('fixtures/day9/input.txt')
    assert_equal 1743490457, num
  end
end

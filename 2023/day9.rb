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

  def test_part1
    num = part1('fixtures/day9/input.txt')
    assert_equal 1743490457, num
  end

  def prev_val(values)
    # puts "RCVD: #{values.inspect}"
    return 0 if values.all?(0)

    diffs = values.each_cons(2).map { |(a, b)| b - a }

    # p diffs
    pv = prev_val(diffs)
    # puts "PV: #{pv}"

    # puts "PV2: #{values.first - pv} -- #{values.first} + #{pv}"
    values.first - pv
  end

  def test_sample_1_rev
    assert_equal 5, prev_val(%w{10  13  16  21  30  45}.map(&:to_i))
  end

  def part2(filename)
    values = parse(filename)
    values.sum do |vs|
      prev_val(vs)
    end
  end

  def test_part2
    num = part2('fixtures/day9/input.txt')
    assert_equal 1053, num
  end
end

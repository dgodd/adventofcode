# frozen_string_literal: true

require 'minitest/autorun'
require 'minitest/focus'

class Day8 < Minitest::Test
  def parse(filename)
    lines = File.read(filename).split("\n")
    sequence = lines.shift.split('').map { |m| m == 'L' ? 0 : 1 }
    lines.shift
    data = {}
    lines.each do |l|
      m = l.match(/(.*)\s+=\s+\((.*),\s+(.*)\)/)
      data[m[1]] = [m[2], m[3]]
    end
    [sequence, data]
  end

  def part1(filename)
    (seq, data) = parse(filename)
    pos = 'AAA'
    (0..).each do |steps|
      instr = seq[steps % seq.length]
      pos = data[pos][instr] 
      return steps + 1 if pos == 'ZZZ'
    end
  end

  def test_sample1a
    num = part1('fixtures/day8/sample_a.txt')
    assert_equal 2, num
  end

  def test_sample1b
    num = part1('fixtures/day8/sample_b.txt')
    assert_equal 6, num
  end

  def test_part1
    num = part1('fixtures/day8/input.txt')
    assert_equal 20221, num
  end

  def lcm(numbers)
    x = numbers.max
    out = x
    until numbers.all? { |n| out % n == 0 }
      out += x
    end
    out
  end

  def part2(filename)
    (seq, data) = parse(filename)
    ps = data.keys.grep(/A$/)
    loop_sizes = ps.map do |pos|
      steps = 0
      while !pos.match(/Z$/)
        instr = seq[steps % seq.length]
        pos = data[pos][instr]
        steps += 1
      end
      steps
    end
    lcm(loop_sizes)
  end

  def test_sample2
    num = part2('fixtures/day8/sample2.txt')
    assert_equal 6, num
  end

  def test_part2
    num = part2('fixtures/day8/input.txt')
    assert_equal 14616363770447, num
  end
end

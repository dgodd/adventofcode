require 'minitest/autorun'
require 'minitest/focus'

class Day6 < Minitest::Test
  def parse(filename)
    m = File.read(filename).match(/Time:\s+(.*)\nDistance:\s+(.*)/)
    time = m[1].split(/\s+/)
    distance = m[2].split(/\s+/)
    [time, distance]
  end

  def part1(filename)
    (time, distance) = parse(filename)
        time = time.map(&:to_i)
        distance = distance.map(&:to_i)
    races = time.zip(distance)
    races.map do |race|
      # p race
      (0..race[0]).select do |speed|
        time = race[0] - speed
        dist = speed * time
        # p dist
        dist > race[1]
      end.length
    end.reduce(:*)
  end

  def test_sample1
    num = part1('fixtures/day6/sample.txt')
    assert_equal 288, num
  end

  def test_part1
    num = part1('fixtures/day6/input.txt')
    assert_equal 771628, num
  end

  def part2(filename)
    (time, distance) = parse(filename)
    time = [time.reduce(:+).to_i]
    distance = [distance.reduce(:+).to_i]
    races = time.zip(distance)
    races.map do |race|
      # p race
      (0..race[0]).select do |speed|
        time = race[0] - speed
        dist = speed * time
        # p dist
        dist > race[1]
      end.length
    end.reduce(:*)
  end

  def test_sample2
    num = part2('fixtures/day6/sample.txt')
    assert_equal 71503, num
  end

  def test_part2
    num = part2('fixtures/day6/input.txt')
    assert_equal 27363861, num
  end
end

require 'minitest/autorun'

class Day5 < Minitest::Test
  def parse(filename)
    maps = []
    curr_map = nil
    lines = File.read(filename).split("\n")
    seeds = lines.shift.gsub(/^seeds:\s+/,'').split(/\s+/).map(&:to_i)
    lines.each do |l|
      next if l.match(/^\s*$/)
      if l.match(/ map:/)
        maps << curr_map if curr_map
        curr_map = []
      else
        curr_map << l.split(/\s+/).map(&:to_i)
      end
    end
    [seeds, maps]
  end

  def part1_map1(seed, map)
    map.each do |(d, s, l)|
      if seed >= s && seed < (s + l)
        seed = d + (seed - s)
        return seed
      end
    end
    seed
  end

  def part1_map(seed, maps)
    maps.each do |map|
      seed = part1_map1(seed, map)
    end
    seed
  end

  def part1(filename)
    (seeds, maps) = parse(filename)
    seeds.map do |seed|
      part1_map(seed, maps)
    end.min
  end

  def test_part1_map
    assert_equal 10, part1_map(10, [])
    assert_equal 10, part1_map(10, [[[0, 5, 2]]])
    assert_equal 5, part1_map(10, [[[0, 5, 20]]])
    assert_equal 17, part1_map(22, [[[0, 5, 20], [20, 3, 10]]])
  end

  def test_sample1
    num = part1('fixtures/day5/sample.txt')
    assert_equal 35, num
  end

  def test_part1
    num = part1('fixtures/day5/input.txt')
    assert_equal 318171373, num
  end
end

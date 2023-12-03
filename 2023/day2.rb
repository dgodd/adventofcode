require 'minitest/autorun'

class Day2 < Minitest::Test
  NUM = { 'red' => 12, 'green' => 13, 'blue' => 14 }

  def part1(filename)
    lines = File.read(filename).split("\n")
    games = lines.map do |l|
      game_num = l.match(/^Game (\d+): /)[1].to_i
      games = l.gsub!(/^Game (\d+): /, '').split(/;\s*/).map { _1.split(/,\s*/) }
      games = games.map { |g| g.map { m = _1.match(/^(\d+) (red|green|blue)/); [m[2], m[1].to_i] } }
      [game_num, games]
    end
    # puts games.map(&:inspect).join("\n")
    possible = games.select do |(_, games)|
      games.all? do |game|
        # puts "  -- #{game.inspect}"
        game.all? { |(name, num)| NUM[name] >= num }
      end
    end
    # puts possible.map(&:inspect).join("\n")
    possible.map { _1[0] }.sum
  end

  def test_sample1
    num = part1('fixtures/day2/sample1.txt')
    assert_equal num, 8
  end

  def test_part1
    num = part1('fixtures/day2/input.txt')
    assert_equal num, 2771
  end
end

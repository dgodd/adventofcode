require 'minitest/autorun'
require 'minitest/focus'

class Day10 < Minitest::Test
  Pos = Struct.new(:x, :y, keyword_init: true) do
    def north = dup.tap { |p| p.y -= 1 }
    def east = dup.tap { |p| p.x += 1 }
    def south = dup.tap { |p| p.y += 1 }
    def west = dup.tap { |p| p.x -= 1 }

    def to_s = "#{y},#{x}"
  end

  def parse(filename)
    File.read(filename).split(/\n/).map { _1.split('') }
  end

  def find_start(board)
    board.each.with_index do |row, y|
      row.each.with_index do |c, x|
        return Pos.new(x: x, y: y) if c == 'S'
      end
    end
    raise 'no start'
  end

  def charon(board, pos)
    retun nil if pos.y < 0 || pos.x < 0
    board[pos.y][pos.x]
  end

  def possibilities(board, pos)
    out = []
    north = charon(board, pos.north)
    out << pos.north if north == '|' || north == 'F' || north == '7'

    east = charon(board, pos.east)
    out << pos.east if east == '-' || east == '7' || east == 'L'

    south = charon(board, pos.south)
    out << pos.south if south == '|' || south == 'J' || south == 'L'

    west = charon(board, pos.west)
    out << pos.west if west == '-' || west == 'F' || west == 'L'

    out
  end

  def possibilities2(board, pos)
    case charon(board, pos)
    when '|'
      [pos.north, pos.south]
    when '-'
      [pos.east, pos.west]
    when 'F'
      [pos.south, pos.east]
    when 'J'
      [pos.north, pos.west]
    when 'L'
      [pos.north, pos.east]
    when '7'
      [pos.south, pos.west]
    else
      raise 'woops'
    end
  end

  def half_loop(board)
    start = find_start(board)
    pos = possibilities(board, start).first
    loop = [start, pos]
    loop do
      ps = possibilities2(board, pos)
      pos = (ps - loop).first
      break if pos == start || pos == nil
      loop << pos
    end
    loop.length / 2
  end

  def test_sample_1
    assert_equal 4, half_loop([
      '.....',
      '.S-7.',
      '.|.|.',
      '.L-J.',
      '.....',
    ].map { _1.split('') })
  end

  def part1(filename)
    half_loop(parse(filename))
  end

  def test_part1
    num = part1('fixtures/day10/input.txt')
    assert_equal 6773, num
  end
end

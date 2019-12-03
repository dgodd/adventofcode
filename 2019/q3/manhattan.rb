class Manhattan
  Pos = Struct.new(:x, :y, :num)

  def initialize(arr)
    @a = {}
    moves(arr) do |pos, num|
      @a[pos.x] ||= {}
      @a[pos.x][pos.y] = num
    end
  end

  def intersections(arr)
    intersections = []
    moves(arr) do |pos, num|
      other = @a[pos.x][pos.y] if @a[pos.x]
      intersections << pos.dup.tap { |p| p.num = num + other } if other
    end
    intersections
  end

  def closest_intersection(arr)
    intersections(arr).map do |pos|
      pos.x.abs + pos.y.abs
    end.sort.first
  end

  def shortest_intersection(arr)
    intersections(arr).map do |pos|
      pos.num
    end.sort.first
  end

  private

  def moves(arr)
    pos = Pos.new(0, 0)
    num = 0
    arr.each do |move|
      direction = move[0]
      amount = move[1..].to_i
      case direction
      when 'R' then
        (1..amount).each { pos.x += 1; num += 1; yield pos, num }
      when 'L' then
        (1..amount).each { pos.x -= 1; num += 1; yield pos, num }
      when 'U' then
        (1..amount).each { pos.y += 1; num += 1; yield pos, num }
      when 'D' then
        (1..amount).each { pos.y -= 1; num += 1; yield pos, num }
      else raise "Bad direction: #{move}"
      end
    end
  end
end

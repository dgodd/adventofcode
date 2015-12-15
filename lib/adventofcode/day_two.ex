defmodule Adventofcode.DayTwo do
  def squarefeet(l,w,h) do
    2*l*w + 2*w*h + 2*h*l + Enum.min([l*w, w*h, h*l])
  end

  def squarefeet(str) do
    [l,w,h] = Regex.split(~r/x/, str)
    # [l,w,h] = Enum.map([l,w,h], fn(x) -> x * 2 end)
    # [l,w,h] = Enum.map([l,w,h], fn(x) -> Integer.parse(x) end)
    {l, _} = Integer.parse(l)
    {w, _} = Integer.parse(w)
    {h, _} = Integer.parse(h)

    squarefeet(l,w,h)
  end
end

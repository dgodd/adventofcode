#!/usr/bin/env ruby
require 'fileutils'
require 'nokogiri'
require 'excon'
require 'reverse_markdown'

# res = Excon.get('https://adventofcode.com/2023', headers: { Cookie: "session=#{ENV['AOC_COOKIE']}" })
# doc = Nokogiri::HTML(res.body)
# days = doc.css('pre.calendar a').map { |a| a[:href] }

day = '3'
res = Excon.get("https://adventofcode.com/2023/day/#{day}", headers: { Cookie: "session=#{ENV['AOC_COOKIE']}" })
doc = Nokogiri::HTML(res.body)
FileUtils.mkdir_p("fixtures/day#{day}")
txt = ReverseMarkdown.convert(doc.css('article.day-desc').to_s)
puts txt
File.write("fixtures/day#{day}/README.txt", txt)
File.write("fixtures/day#{day}/sample1.txt", doc.css('article.day-desc > pre > code').text)

res = Excon.get("https://adventofcode.com/2023/day/#{day}/input", headers: { Cookie: "session=#{ENV['AOC_COOKIE']}" })
File.write("fixtures/day#{day}/input.txt", res.body)

system('git', 'add', "fixtures/day#{day}")

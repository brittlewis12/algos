require_relative 'graph'
require_relative 'dijkstra'

graph = Graph.from_file('DijkstraData.txt')

lengths = Dijkstra.find_distances(s: 1, graph: graph)

puts "Distances from 1 to:"
puts "  7 => #{lengths[7]}"
puts " 37 => #{lengths[37]}"
puts " 59 => #{lengths[59]}"
puts " 82 => #{lengths[82]}"
puts " 99 => #{lengths[99]}"
puts "115 => #{lengths[115]}"
puts "133 => #{lengths[133]}"
puts "165 => #{lengths[165]}"
puts "188 => #{lengths[188]}"
puts "197 => #{lengths[197]}"

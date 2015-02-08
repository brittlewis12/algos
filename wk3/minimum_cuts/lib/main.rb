require_relative 'minimum_cuts'

graph = MinimumCuts.graph_from_file('KargerMinCut.txt')

puts MinimumCuts.find(graph)

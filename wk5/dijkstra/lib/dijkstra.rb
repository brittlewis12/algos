require 'set'

module Dijkstra
  def self.find_distances(s:, graph:)
    graph.freeze
    vertices = Set.new graph.keys

    raise ArgumentError, 's must be a vertex in graph' unless vertices.include?(s)

    lengths = {}
    lengths[s] = 0
    processed = Set.new [s]

    while processed != vertices do
      min_score = Float::INFINITY
      min_vertex = nil

      processed.each do |v|
        graph[v].each do |w, length|
          unless processed.include?(w)
            score = lengths[v] + length
            if score < min_score
              min_score = score
              min_vertex = w
            end
          end
        end
      end

      processed << min_vertex
      lengths[min_vertex] = min_score
    end

    lengths
  end
end

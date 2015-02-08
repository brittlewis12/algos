module MinimumCuts
  class << self
    def find(graph)
      graph.freeze
      num_vertices = graph.keys.count
      num_trials = (num_vertices ** 2 * Math.log(num_vertices)).ceil

      (1..num_trials).map {
        Thread.new { find_cuts(graph.dup) }
      }.map(&:value).min
    end

    def graph_from_file(path)
      raw_input = IO.readlines(path)
        .map(&:split)
        .map { |row| row.map(&:to_i) }

      graph = {}
      raw_input.each do |row|
        graph[row.shift] = row
      end

      graph
    end

    private

    def find_cuts(graph)
      vertices = graph.keys
      return graph[vertices[0]].count if vertices.count == 2

      i = rand(vertices.count - 1)
      u = vertices[i]
      edges = graph[u]
      j = rand(edges.count - 1)
      v = edges[j]

      # add v values to u
      graph[u] = graph[u] + graph[v]

      # find all v references in other edges and replace with u
      graph.keys.each do |vertex|
        new_edges = graph[vertex].map do |edge|
          edge == v ? u : edge
        end
        graph[vertex] = new_edges
      end

      # delete v from graph
      graph.delete(v)

      # delete u from u's own edges (self loops)
      graph[u] = graph[u].reject { |edge| edge == u }

      find_cuts(graph)
    end
  end
end

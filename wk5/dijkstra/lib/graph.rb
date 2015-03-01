module Graph
  def self.from_file(path)
    adjacency_list = {}

    IO.readlines(path).each do |line|
      split = line.strip.split
      vertex = split.shift.to_i
      pairs = split.map { |pair| pair.split(',').map(&:to_i) }
      adjacency_list[vertex] = pairs
    end

    adjacency_list
  end
end

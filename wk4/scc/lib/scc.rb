require 'pry-byebug'

module SCC
  def self.compute_from_file(file)
    edges = File.readlines(file).map { |line| line.strip.split.map(&:to_i) }.freeze
    sccs = SCC.compute(edges: edges)
    five_largest = sccs.values.map(&:size).sort { |a, b| b <=> a }.take(5)

    return five_largest
  end

  def self.compute(edges:)
    $explored = Set.new
    $current_finishing_time = 0 # for finishing times
    $finishing_times = Hash.new # finishing_time => vertex
    compute_rev(edges: edges)

    $explored = Set.new
    $current_leader = nil
    $sccs = Hash.new(Set.new)
    compute_scc(edges: edges)

    $sccs
  end

  def self.compute_rev(edges:)
    rev_adjacency_list = Hash.new([])
    edges.each do |tail, head|
      rev_adjacency_list[head] += [tail]
    end
    rev_adjacency_list.freeze

    i = edges.flatten.uniq.count
    while i >= 1 do
      unless $explored.include?(i)
        dfs_loop_rev(adjacency_list: rev_adjacency_list, s: i)
      end
      i -= 1
    end
  end

  def self.compute_scc(edges:)
    adjacency_list = Hash.new([])
    edges.each do |tail, head|
      adjacency_list[tail] += [head]
    end

    i = edges.flatten.uniq.count
    while i >= 1 do
      # i is a finishing time
      s = $finishing_times[i]
      unless $explored.include?(s)
        $current_leader = s
        dfs_loop(adjacency_list: adjacency_list, s: s)
      end
      i -= 1
    end
  end

  # set finishing times
  def self.dfs_loop_rev(adjacency_list:, s:)
    $explored.add(s)
    next_edges = adjacency_list[s]
    next_edges.each do |v|
      unless $explored.include?(v)
        puts "unexplored: #{v}"
        dfs_loop_rev(adjacency_list: adjacency_list, s: v)
      end
    end
    $current_finishing_time += 1
    $finishing_times[$current_finishing_time] = s
  end

  # set leaders
  def self.dfs_loop(adjacency_list:, s:)
    $explored.add(s)
    $sccs[$current_leader] += [s]
    next_edges = adjacency_list[s]
    next_edges.each do |v|
      unless $explored.include?(v)
        dfs_loop(adjacency_list: adjacency_list, s: v)
      end
    end
  end
end

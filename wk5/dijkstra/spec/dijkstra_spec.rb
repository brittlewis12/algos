require_relative '../lib/dijkstra'
require_relative '../lib/graph'

describe Dijkstra do
  describe ".find_distances" do
    let(:lengths) { described_class.find_distances(s: start, graph: graph) }

    context "when given start vertex is not in graph" do
      let(:start) { 1 }
      let(:graph) { {} }

      it "raises an Argument Error" do
        expect { lengths }.to raise_error ArgumentError, 's must be a vertex in graph'
      end
    end

    context "with test case 1" do
      let(:start) { 1 }
      let(:graph) { Graph.from_file('test1.txt') }

      it "returns the shortest path length to each reachable vertex in the graph" do
        expect(lengths[1]).to eq(0)
        expect(lengths[2]).to eq(3)
        expect(lengths[3]).to eq(3)
        expect(lengths[4]).to eq(5)
      end
    end

    context "with test case 2" do
      let(:start) { 13 }
      let(:graph) { Graph.from_file('test2.txt') }

      it "returns the shortest path length to each reachable vertex in the graph" do
        expect(lengths[5]).to eq(26)
      end
    end

    context "with test case 3" do
      let(:start) { 28 }
      let(:graph) { Graph.from_file('test3.txt') }

      it "returns the shortest path length to each reachable vertex in the graph" do
        expect(lengths[6]).to eq(9)
      end
    end

    context "with DijkstraData.txt" do
      let(:start) { 1 }
      let(:graph) { Graph.from_file('DijkstraData.txt') }

      it "returns the shortest path length to each reachable vertex in the graph" do
        shortest_distances = lengths

        expect(shortest_distances[10]).to eq(3205)
        expect(shortest_distances[30]).to eq(2303)
        expect(shortest_distances[50]).to eq(3152)
        expect(shortest_distances[80]).to eq(982 )
        expect(shortest_distances[90]).to eq(2018)
        expect(shortest_distances[110]).to eq(2317)
        expect(shortest_distances[130]).to eq(1820)
        expect(shortest_distances[160]).to eq(2403)
        expect(shortest_distances[180]).to eq(3027)
        expect(shortest_distances[190]).to eq(2596)
      end
    end
  end
end

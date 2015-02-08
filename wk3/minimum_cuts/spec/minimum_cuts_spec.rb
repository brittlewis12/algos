require_relative '../lib/minimum_cuts'

describe MinimumCuts do
  describe "#find" do
    context "when mix cuts should be two" do
      it "runs randomized contraction to find the minimum cut" do
        graph = {
          1 => [2, 5, 6].shuffle,    #            min cut
          2 => [1, 3, 5, 6].shuffle, #  1        2   ||   3        4           1&2            3        4
          3 => [2, 4, 7, 8].shuffle, #  *--------*--------*--------*            *-------------*--------*
          4 => [3, 7, 8].shuffle,    #  | \    / |   ||   | \    / |          // \\           | \    / |
          5 => [1, 2, 6].shuffle,    #  |   \/   |   ||   |   \/   |   =>    //   \\          |   \/   |
          6 => [1, 2, 5, 7].shuffle, #  | /    \ |   ||   | /    \ |        //     \\         | /    \ |
          7 => [3, 4, 6, 8].shuffle, #  *--------*--------*--------*        *--------*--------*--------*
          8 => [3, 4, 7].shuffle,    #  5        6   ||   7        8        5        6        7        8
        }
        count = MinimumCuts.find(graph)
        expect(count).to eq 2
      end
    end

    context "when min cuts should be one" do
      it "runs randomized contraction to find the minimum cut" do
        graph = {
          1 => [2, 5, 6].shuffle,    #            min cut
          2 => [1, 3, 5, 6].shuffle, #  1        2   ||   3        4
          3 => [2, 4, 7, 8].shuffle, #  *--------*--------*--------*
          4 => [3, 7, 8].shuffle,    #  | \    / |   ||   | \    / |
          5 => [1, 2, 6].shuffle,    #  |   \/   |   ||   |   \/   |
          6 => [1, 2, 5].shuffle,    #  | /    \ |   ||   | /    \ |
          7 => [3, 4, 8].shuffle,    #  *--------*   ||   *--------*
          8 => [3, 4, 7].shuffle,    #  5        6   ||   7        8
        }
        count = MinimumCuts.find(graph)
        expect(count).to eq 1
      end
    end

    context "when min cuts should be three" do
      it "runs randomized contraction to find the minimum cut" do
        graph = {
          1 => [2, 5, 6].shuffle,       #            min cut
          2 => [1, 3, 5, 6, 7].shuffle, #  1        2   ||   3        4
          3 => [2, 4, 6, 7, 8].shuffle, #  *--------*--------*--------*
          4 => [3, 7, 8].shuffle,       #  | \    / | \ ||  /| \    / |
          5 => [1, 2, 6].shuffle,       #  |   \/   |  \||/  |   \/   |
          6 => [1, 2, 3, 5].shuffle,    #  | /    \ | / || \ | /    \ |
          7 => [2, 3, 4, 8].shuffle,    #  *--------*   ||   *--------*
          8 => [3, 4, 7].shuffle,       #  5        6   ||   7        8
        }
        count = MinimumCuts.find(graph)
        expect(count).to eq 3
      end
    end

    context "reading input from a text file" do
      let(:graph) { MinimumCuts.graph_from_file('TestCase1.txt') }

      it "converts the file into the expected graph hash" do
        expect(graph).to be_an_instance_of(Hash)
        expect(graph.keys.count).to eq 40
      end

      it "returns the correct number of min cuts" do
        count = MinimumCuts.find(graph)
        expect(count).to eq 3
      end
    end
  end
end

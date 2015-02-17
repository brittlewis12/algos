require_relative '../lib/scc'

describe SCC do
  describe "#compute_from_file" do
    subject { described_class.compute_from_file filename }

    context "with text_1.txt input" do
      let(:filename) { 'test_1.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [3,3,3]
      end
    end

    context "with text_2.txt input" do
      let(:filename) { 'test_2.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [3,3,2]
      end
    end

    context "with text_3.txt input" do
      let(:filename) { 'test_3.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [3,3,1,1]
      end
    end

    context "with text_4.txt input" do
      let(:filename) { 'test_4.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [7,1]
      end
    end

    context "with text_5.txt input" do
      let(:filename) { 'test_5.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [6,3,2,1]
      end
    end

    context "with text_6.txt input" do
      let(:filename) { 'test_6.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [3,2,2,2,1]
      end
    end

    context "with text_7.txt input" do
      let(:filename) { 'test_7.txt' }
      it "finds the five largest strongly connected components" do
        expect(subject).to eq [5,4,3,2,1]
      end
    end
  end
end

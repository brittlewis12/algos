require_relative '../lib/quick_sort'

describe QuickSort do
  describe "#sort" do
    context "with no pivot specified" do
      subject { described_class.sort array }

      context "an input array of 10 integers" do
        let(:array) { [8, 3, 2, 10, 6, 5, 1, 9, 4, 7] }
        it { is_expected.to eq [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
      end

      context "an input array of 100 integers" do
        let(:array) { (1..100).to_a.shuffle }
        it { is_expected.to eq (1..100).to_a }
      end

      context "an input array of 1000 integers" do
        let(:array) { (1..1000).to_a.shuffle }
        it { is_expected.to eq (1..1000).to_a }
      end
    end

    context "with a custom pivot" do
      subject { described_class.sort array, -1 }
      let(:array) { (1..1000).to_a.shuffle }
      it { is_expected.to eq (1..1000).to_a }
    end
  end

  describe "#count_comparisons" do
    context "with no custom pivot" do
      subject { described_class.count_comparisons array }

      context "with 2 elements" do
        let(:array) { [2, 1] }
        it { is_expected.to eq [[1, 2], 1] }
      end
    end

    context "with a custom pivot" do
      subject { described_class.count_comparisons array, 1 }

      context "with 2 elements" do
        let(:array) { [2, 1] }
        it { is_expected.to eq [[1, 2], 1] }
      end
    end
  end

  describe "#count_median_comparisons" do
    subject { described_class.count_median_comparisons array }

    context "with ten.txt test input" do
      let(:array)      { IO.readlines('ten.txt').map(&:to_i) }
      it "counts the correct number of comparisons" do
        expect(subject[1]).to eq 21
      end
    end

    context "with hundred.txt test input" do
      let(:array)  { IO.readlines('hundred.txt').map(&:to_i) }
      it "counts the correct number of comparisons" do
        expect(subject[1]).to eq 518
      end
    end

    context "with thousand.txt test input" do
      let(:array) { IO.readlines('thousand.txt').map(&:to_i) }
      it "counts the correct number of comparisons" do
        expect(subject[1]).to eq 8921
      end
    end
  end
end

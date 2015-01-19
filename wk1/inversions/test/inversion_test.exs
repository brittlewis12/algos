ExUnit.start()
Code.require_file("inversion.ex", "lib")

defmodule InversionTest do
  use ExUnit.Case

  test "returns 0 when all integers are sequential" do
    input = Enum.to_list(1..10)
    assert Inversion.count(input) == 0
  end

  test "handles odd-length inputs" do
    input = [1, 3, 2] # inversions = [{3, 2}]
    assert Inversion.count(input) == 1
  end

  test "returns 1 when only two sequential integers are inverted" do
    input = [1, 3, 2, 4, 5, 6] # inversions = [{3, 2}]
    assert Inversion.count(input) == 1
  end

  test "when there are only split inversions" do
    input = [1, 3, 5, 2, 4, 6] # inversions = [{3, 2}, {5, 2}, {5, 4}]
    assert Inversion.count(input) == 3
  end

  test "handles large inputs with maximum possible inversions" do
    n = 100000
    input = Enum.to_list(1..n) |> Enum.reverse # n(n-1)/2 inversions
    assert Inversion.count(input) == n*(n-1)/2
  end

end

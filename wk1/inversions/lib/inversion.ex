defmodule Inversion do

  def count(list) when is_list(list) do
    {_, count} = sort_and_count(list)
    count
  end

  defp sort_and_count([]), do: {[], 0}
  defp sort_and_count([head]), do: {[head], 0}
  defp sort_and_count(list) do
    mid = length(list) |> div(2)

    [{left, count1}, {right, count2}] =
      Enum.split(list, mid)
      |> Tuple.to_list
      |> Enum.map(&sort_and_count(&1))

    inv_counts = [count1, count2]
    {sorted, count} = count_split_invs(left, right)

    [count | inv_counts]
    |> Enum.reduce(&+/2)
    |> (&({sorted, &1})).()
  end

  defp count_split_invs(left, right) do
    split_invs(left, right, [], 0)
  end

  defp split_invs([], right, merged, count) do
    {Enum.reverse(merged, right), count}
  end
  defp split_invs(left, [], merged, count) do
    {Enum.reverse(merged, left), count}
  end
  defp split_invs(l = [lhead | ltail], r = [rhead | rtail], merged, count) do
    if lhead < rhead do
      split_invs(ltail, r, [lhead | merged], count)
    else
      split_invs(l, rtail, [rhead | merged], (count + length(l)))
    end
  end

end

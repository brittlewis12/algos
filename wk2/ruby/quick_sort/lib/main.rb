require_relative 'quick_sort'

integers = IO.readlines('IntegerInput.txt').map(&:to_i)

result = QuickSort.count_comparisons(integers.clone)
puts "Comparisons with first: #{result[1]}"

result = QuickSort.count_comparisons(integers.clone, -1)
puts "Comparisons with last: #{result[1]}"

result = QuickSort.count_median_comparisons(integers.clone)
puts "Comparisons with median-of-three: #{result[1]}"

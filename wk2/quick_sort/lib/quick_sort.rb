module QuickSort
  class << self
    def sort array, custom_pivot = 0
      return array if array.length <= 1

      result = partition(array, custom_pivot)
      left = sort(result[:left])
      pivot = [result[:pivot]].compact
      right = sort(result[:right])

      left + pivot + right
    end

    def count_comparisons array, custom_pivot = 0, count = 0
      return [array, count] if array.length <= 1

      count += array.length - 1

      result = partition(array, custom_pivot)
      left, count = count_comparisons(result[:left], custom_pivot, count)

      pivot = [result[:pivot]].compact

      right, count = count_comparisons(result[:right], custom_pivot, count)

      [left + pivot + right, count]
    end

    def count_median_comparisons array, count = 0
      return [array, count] if array.length <= 1

      count += array.length - 1

      result = median_partition(array)
      left, count = count_median_comparisons(result[:left], count)

      pivot = [result[:pivot]].compact

      right, count = count_median_comparisons(result[:right], count)

      [left + pivot + right, count]
    end

    private

    def partition array, custom_pivot
      return { left: [], pivot: array[0], right: [] } if array.length <= 1

      array[0], array[custom_pivot] = array[custom_pivot], array[0] unless custom_pivot == 0

      pivot, i, j = 0, 1, 1
      while j < array.length do
        if array[j] < array[pivot]
          array[i], array[j] = array[j], array[i] if i != j
          j += 1
          i += 1
        else
          j += 1
        end
      end
      array[pivot], array[i-1] = array[i-1], array[pivot]
      return {
        pivot: array[i-1],
        left: array.select.each_with_index {|n, idx| idx < i - 1 },
        right: array.select.each_with_index {|n, idx| idx > i - 1 },
      }
    end

    def median_partition array
      return { left: [], pivot: array[0], right: [] } if array.length <= 1

      pivot = 0
      first, last = array[0], array[-1]
      middle_idx = (array.length/2.0).ceil - 1
      middle = array[middle_idx]

      if first <= middle && middle <= last
        pivot = middle_idx
      elsif last <= middle && middle <= first
        pivot = middle_idx
      elsif last <= first && first <= middle
        pivot = 0
      elsif middle <= first && first <= last
        pivot = 0
      elsif last
        pivot = -1
      end

      array[0], array[pivot] = array[pivot], array[0]

      pivot, i, j = 0, 1, 1
      while j < array.length do
        if array[j] < array[pivot]
          array[i], array[j] = array[j], array[i] if i != j
          j += 1
          i += 1
        else
          j += 1
        end
      end
      array[pivot], array[i-1] = array[i-1], array[pivot]
      return {
        pivot: array[i-1],
        left: array.select.each_with_index {|n, idx| idx < i - 1 },
        right: array.select.each_with_index {|n, idx| idx > i - 1 },
      }
    end
  end
end

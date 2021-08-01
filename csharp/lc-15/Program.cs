using System;
using System.Collections.Generic;

namespace lc_15
{
    class Program
    {
        static void Main(string[] args)
        {
            var s = new Solution();

            var n1 = new int[] { -1, 0, 1, 2, -1, -4 };
            var n2 = new int[] { };
            var n3 = new int[] { 0 };
            var n4 = new int[] { 1, -1, -1, 0 };

            // Expecting [[-1, -1, 2], [-1, 0, 1]]
            PrintList(s.ThreeSum(n1));
			// Expecting []
            PrintList(s.ThreeSum(n2));
			// Expecting []
            PrintList(s.ThreeSum(n3));
			// Expecting [[-1, 0, 1]]
			PrintList(s.ThreeSum(n4));
        }

        static void PrintList(IList<IList<int>> list)
        {
            Console.Write("[");
            var isFirst = true;
            foreach (var nums in list)
            {
                if (!isFirst)
                {
                    Console.Write(", ");
                }
				Console.Write("[");
                Console.Write(String.Join(", ", nums));
				Console.Write("]");
                isFirst = false;
            }
            Console.WriteLine("]");
        }
    }

    class Solution
    {
        public IList<IList<int>> ThreeSum(int[] nums)
        {
            IList<IList<int>> result = new List<IList<int>>();
            Array.Sort(nums);

            for (var i = 0; i < nums.Length; i++)
            {
                if (nums[i] > 0)
                {
                    return result;
                }

                if (i > 0 && nums[i] == nums[i - 1])
                {
                    continue;
                }

                var left = i + 1;
                var right = nums.Length - 1;
                while (right > left)
                {
                    var sum = nums[i] + nums[left] + nums[right];
                    if (sum < 0)
                    {
                        left++;
                    }
                    else if (sum > 0)
                    {
                        right--;
                    }
                    else
                    {
                        result.Add(new List<int> { nums[i], nums[left], nums[right] });

                        while (right > left && nums[right] == nums[right - 1])
                        {
                            right--;
                        }
                        while (right > left && nums[left] == nums[left + 1])
                        {
                            left++;
                        }

                        left++;
                        right--;
                    }
                }
            }

            return result;
        }
    }
}

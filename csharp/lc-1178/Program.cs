using System;
using System.Collections.Generic;

namespace lc_1178
{
    class Program
    {
        static void Main(string[] args)
        {
            var words1 = new string[] { "aaaa", "asas", "able", "ability", "actt", "actor", "access" };
            var puzzles1 = new string[] { "aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz" };

            var solution = new Solution();

            // Expecting [1, 1, 3, 2, 4, 0]
            PrintList(solution.FindNumOfValidWords(words1, puzzles1));
        }

        static void PrintList<T>(IList<T> list)
        {
            Console.Write("[");
            var len = list.Count;
            var e = list.GetEnumerator();
            var isFirst = true;
            while (e.MoveNext())
            {
                if (!isFirst)
                {
                    Console.Write(", ");
                }
                Console.Write(e.Current);
                isFirst = false;
            }
            Console.WriteLine("]");
        }
    }
}

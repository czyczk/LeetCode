using System;

namespace lc_59
{
    class Program
    {
        static void Main(string[] args)
        {
            var s = new Solution();
            // Expecting [[1, 2, 3], [8, 9, 4], [7, 6, 5]]
            PrintMatrix(s.GenerateMatrix(3));
        }

        private static void PrintMatrix(int[][] matrix)
        {
            Console.Write("[");
            var isFirst = true;
            foreach (var line in matrix)
            {
                if (!isFirst)
                {
                    Console.Write(", ");
                }
                isFirst = false;

                Console.Write("[");
                var isFirstInLine = true;
                foreach (var num in line)
                {
                    if (!isFirstInLine)
                    {
                        Console.Write(", ");
                    }
                    isFirstInLine = false;

                    Console.Write(num);
                }

                Console.Write("]");
            }

            Console.WriteLine("]");
        }
    }
}

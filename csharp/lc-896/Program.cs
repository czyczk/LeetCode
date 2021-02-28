using System;

namespace lc_896
{
    class Program
    {
        static void Main(string[] args)
        {
            var a1 = new int[] { 1, 2, 2, 3 };
            var a2 = new int[] { 6, 5, 4, 4 };
            var a3 = new int[] { 1, 3, 2 };
            var a4 = new int[] { 1, 2, 4, 5 };
            var a5 = new int[] { 1, 1, 1 };

            var s = new Solution();
            // Expecting true
            Console.WriteLine(s.IsMonotonic(a1));
            // Expecting true
            Console.WriteLine(s.IsMonotonic(a2));
            // Expecting false
            Console.WriteLine(s.IsMonotonic(a3));
            // Expecting true
            Console.WriteLine(s.IsMonotonic(a4));
            // Expecting true
            Console.WriteLine(s.IsMonotonic(a5));
        }
    }
}

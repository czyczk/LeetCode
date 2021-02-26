using System;
using System.Collections.Generic;
using System.Numerics;

namespace lc_1178
{
    public class Solution
    {
        public IList<int> FindNumOfValidWords(string[] words, string[] puzzles)
        {
            var wordFreqDic = new Dictionary<uint, int>();
            foreach (var word in words)
            {
                var mask = 0u;
                foreach (var ch in word)
                {
                    mask |= (1u << (ch - 'a'));
                }

                if (BitOperations.PopCount(mask) <= 7)
                {
                    wordFreqDic[mask] = wordFreqDic.GetValueOrDefault(mask, 0) + 1;
                }
            }

            var ret = new List<int>();

            foreach (var puzzle in puzzles)
            {
                var mask = 0u;
                var maskFirstCh = 1u << (puzzle[0] - 'a');
                var puzzleSpan = puzzle.AsSpan();
                foreach (var ch in puzzleSpan.Slice(1))
                {
                    mask |= (1u << (ch - 'a'));
                }
                var numMatchingWords = 0;

                var subset = mask;
                do
                {
                    var wordPattern = subset | maskFirstCh;
                    numMatchingWords += wordFreqDic.GetValueOrDefault(wordPattern, 0);
                    subset = (subset - 1) & mask;
                } while (subset != mask);

                ret.Add(numMatchingWords);
            }

            return ret;
        }
    }
}


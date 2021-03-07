using System;
using System.IO;

namespace Day1
{
    public static class Day1
    {
        public static void Main(string[] args)
        {
            var input = Resources.input;

            PartOne(input);
            PartTwo(input);
        }

        private static void PartOne(string input)
        {
            Console.WriteLine($@"Part 1: {SeqSum(input, false)}");
        }

        private static void PartTwo(string input)
        {
            Console.WriteLine($@"Part 2: {SeqSum(input, true)}");
        }

        public static int SeqSum(string seq, bool part2)
        {
            var total = 0;
            for (var i = 0; i < seq.Length; i++)
            {
                var comp = (i + (part2 ? seq.Length / 2 : 1)) % seq.Length;

                if (seq[i] == seq[comp])
                {
                    total += (int)char.GetNumericValue(seq, i);
                }
            }
            return total;
        }
    }
}

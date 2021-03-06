using System;
using System.IO;

namespace Day1
{
    public class Program
    {
        static void Main(string[] args)
        {
            string input = File.ReadAllText("Input/1.txt");

            PartOne(input);
            PartTwo(input);
        }

        static void PartOne(string input)
        {
            Console.WriteLine($"Part 1: {SeqSum(input, false)}");
        }

        static void PartTwo(string input)
        {
            Console.WriteLine($"Part 2: {SeqSum(input, true)}");
        }

        public static int SeqSum(string seq, bool part2)
        {
            int total = 0;
            for (var i = 0; i < seq.Length; i++)
            {
                var comp = (i + (part2 ? seq.Length / 2 : 1)) % seq.Length;

                if (seq[i] == seq[comp])
                {
                    total += (int)Char.GetNumericValue(seq, i);
                }
            }
            return total;
        }
    }
}

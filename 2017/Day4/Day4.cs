using System;
using System.Linq;
using Common;

namespace Day4
{
    class Day4
    {
        static void Main(string[] args)
        {
            var partOne = Input.Lines(4).Where(IsValid).Count();
            var partTwo = Input.Lines(4).Where(IsValidV2).Count();

            Console.WriteLine($@"Part 1: {partOne}");
            Console.WriteLine($@"Part 2: {partTwo}");
        }

        static bool IsValid(string passphrase)
        {
            var words = passphrase.Split(' ');
            return words.Length == words.ToHashSet().Count;
        }

        static bool IsValidV2(string passphrase)
        {
            var words = passphrase.Split(' ');
            var sortedWords = words.Select(w => string.Concat(w.OrderBy(c => c))).ToHashSet();
            return words.Length == sortedWords.Count;
        }
    }
}

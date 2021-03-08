using System;
using System.Collections.Generic;
using System.Linq;

namespace Day2
{
    public class Spreadsheet
    {
        private readonly List<List<int>> _table = new();

        public Spreadsheet(string input)
        {
            foreach (var line in input.Split("\r\n"))
            {
                var numbers = line.Split('\t', ' ');
                var parsed = numbers.Select(int.Parse);
                var list = new List<int>(parsed);
                _table.Add(list);
            }
        }

        public int GetChecksum()
        {
            return _table.Sum(l => Math.Abs(l.Min() - l.Max()));
        }

        public int GetChecksumV2()
        {
            return _table.Sum(GetEvenDivisors);
        }

        private int GetEvenDivisors(List<int> row)
        {
            row.Sort();
            row.Reverse();
            for (var i = 0; i < row.Count; i++)
            for (var j = i + 1; j < row.Count; j++)
                if (row[i] % row[j] == 0)
                {
                    return row[i] / row[j];
                }

            return -1;
        }
    }

    public static class Day2
    {
        private static void Main(string[] args)
        {
            var spreadsheet = new Spreadsheet(Resources.input);

            Console.WriteLine($@"Part 1: {spreadsheet.GetChecksum()}");
            Console.WriteLine($@"Part 2: {spreadsheet.GetChecksumV2()}");
        }
    }
}
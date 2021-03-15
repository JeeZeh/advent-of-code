using System;
using System.Linq;
using Common;

namespace Day5
{
    class Day5
    {
        static void Main(string[] args)
        {
            var instructions = Input.Lines(5).Select(int.Parse).ToArray();

            Console.WriteLine($@"Part 1: {Run(instructions)}");
        }

        static int Run(int[] instructions)
        {
            int steps = 0;

            int ptr = 0;

            while (ptr < instructions.Length)
            {
                ptr = Step(ptr, instructions);
                steps++;
            }

            return steps;
        }

        static int Step(int ptr, int[] instructions)
        {
            var movePtr = instructions[ptr];
            instructions[ptr] += movePtr >= 3 ? -1 : 1;
            ptr += movePtr;

            return ptr;
        }
    }
}

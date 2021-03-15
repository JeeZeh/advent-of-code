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
            Console.WriteLine($@"Part 1: {Run(instructions, AlwaysOne)}");

            instructions = Input.Lines(5).Select(int.Parse).ToArray();
            Console.WriteLine($@"Part 2: {Run(instructions, ThreesACrowd)}");
        }

        static int AlwaysOne(int v)
        {
            return 1;
        }
        
        static int ThreesACrowd(int v)
        {
            return v >= 3 ? -1 : 1;
        }

        static int Run(int[] instructions, Func<int, int> stepIncr)
        {
            int steps = 0;

            int ptr = 0;

            while (ptr < instructions.Length)
            {
                ptr = Step(ptr, instructions, stepIncr);
                steps++;
            }

            return steps;
        }

        static int Step(int ptr, int[] instructions, Func<int, int> incr)
        {
            var movePtr = instructions[ptr];
            instructions[ptr] += incr(movePtr);
            ptr += movePtr;

            return ptr;
        }
    }
}

using System;
using System.Linq;
using Common;


namespace Day3
{
    class Day3
    {
        private static DefaultDictionary<Position, int> grid;

        static (Position, int)[] spiralDirections = {
            (new Position(1, 0), 1),
            (new Position(0, 1), 0),
            (new Position(-1, 0), 1),
            (new Position(0, -1), 0),
        };
        static Position[] diagonals = {
            new Position(0,1),
            new Position(1,1),
            new Position(1,0),
            new Position(1,-1),
            new Position(0,-1),
            new Position(-1, -1),
            new Position(-1, 0),
            new Position(-1, 1),
        };
        static void Main(string[] args)
        {
            var input = int.Parse(Resources.input);

            Console.WriteLine($@"Part 1: {PartOne(input)}");
            Console.WriteLine($@"Part 2: {PartTwo(input)}");
        }

        static int PartOne(int searching)
        {
            SpiralFillUntil(() => grid.ContainsValue(searching), IncrementFill);
            Position match = grid.First(pair => pair.Value == searching).Key;

            return Math.Abs(match.x) + Math.Abs(match.y);
        }

        static int PartTwo(int searching)
        {
            SpiralFillUntil(() => grid.Values.Max() > searching, SurroundingSumFill);

            return grid.First(pair => pair.Value > searching).Value;
        }

        static void SpiralFillUntil(Func<bool> predicate, Func<Position, int> fill)
        {
            Position pos = InitGrid();
            var distance = 0;

            while (!predicate())
            {
                foreach (var (dir, distanceIncrement) in spiralDirections)
                {
                    distance += distanceIncrement;
                    pos = Walk(pos, dir, distance, fill);
                }
            }
        }

        static Position Walk(Position current, Position direction, int distance, Func<Position, int> fill)
        {
            for (var i = 0; i < distance; i++)
            {
                current = current.Add(direction);
                grid[current] = fill(current);
            }

            return current;
        }

        static int IncrementFill(Position pos)
        {
            return grid.Keys.Count + 1;
        }

        static int SurroundingSumFill(Position pos)
        {
            int sum = 0;
            foreach (var dir in diagonals)
            {
                sum += grid[pos.Add(dir)];
            }

            return sum;
        }

        static Position InitGrid()
        {
            Position center = new Position(0, 0);
            grid = new DefaultDictionary<Position, int>(() => 0);
            grid[center] = 1;

            return center;
        }
    }
}

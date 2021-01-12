import { getInputLines } from "../../aoc-helper.ts";
import { Point } from "../../aoc-utils.ts";

const dirs: { [key: string]: Point } = {
  U: new Point(0, -1),
  D: new Point(0, 1),
  L: new Point(-1, 0),
  R: new Point(1, 0),
};

const traverseKeypad = (pos: Point, keypad: any[][], path: string): Point => {
  for (const d of path) {
    let newPos = pos.add(dirs[d]);

    if (keypad[newPos.y][newPos.x] !== null) {
      pos = newPos;
    }
  }

  return pos;
};

const getCode = (start: Point, keypad: any[][], paths: string[]): string => {
  let code = "";

  for (const path of paths) {
    start = traverseKeypad(start, keypad, path);
    code += keypad[start.y][start.x];
  }

  return code;
};

const solve = async () => {
  const keypad = [
    [null, null, null, null, null],
    [null, "1", "2", "3", null],
    [null, "4", "5", "6", null],
    [null, "7", "8", "9", null],
    [null, null, null, null, null],
  ];

  const cursedPad = [
    [null, null, null, null, null, null, null],
    [null, null, null, "1", null, null, null],
    [null, null, "2", "3", "4", null, null],
    [null, "5", "6", "7", "8", "9", null],
    [null, null, "A", "B", "C", null, null],
    [null, null, null, "D", null, null, null],
    [null, null, null, null, null, null, null],
  ];

  const lines = await getInputLines(import.meta.url, "real");

  let wrongCode = getCode(new Point(1, 1), keypad, lines);
  console.log(`(Wrong) code to the bathroom: ${wrongCode}`);

  let rightCode = getCode(new Point(1, 3), cursedPad, lines);
  console.log(`(Wrong) code to the bathroom: ${rightCode}`);
};

export { solve };

if (import.meta.main) {
  solve();
}

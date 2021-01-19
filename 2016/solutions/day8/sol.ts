import { getInputLines } from "../../aoc-helper.ts";
import { copy2d, count2d, create2d, print2d } from "../../aoc-utils.ts";

type LCD = boolean[][];

type RotateArgs = {
  axis: "C" | "R";
  index: number;
  n: number;
};

type CreateArgs = {
  x: number;
  y: number;
};

const rotate = (lcd: LCD, args: RotateArgs): LCD => {
  const { axis, index, n } = args;

  let lcdClone: LCD = copy2d(lcd);
  if (axis === "C") {
    for (const [y, _] of lcd.entries()) {
      lcdClone[(y + n) % lcd.length][index] = lcd[y][index];
    }
    return lcdClone;
  }

  for (const [x, _] of lcd[index].entries()) {
    lcdClone[index][(x + n) % lcd[index].length] = lcd[index][x];
  }

  return lcdClone;
};

const create = (lcd: LCD, args: CreateArgs): LCD => {
  const { x, y } = args;

  for (let i = 0; i < Math.min(lcd.length, y); i++) {
    for (let j = 0; j < Math.min(lcd[0].length, x); j++) {
      lcd[i][j] = true;
    }
  }

  return lcd;
};

/**
 * Hate using (...args: any[]) => void here.
 * I had "(args: RotateArgs) => void" and "(args: CreateArgs) => void"
 * respectively to ensure that whatevery pair of function and args are returned
 * they should match, but the compiler is really not happy about this...
 */
type Instruction =
  | [(...args: any[]) => LCD, RotateArgs]
  | [(...args: any[]) => LCD, CreateArgs];

const parseInstruction = (line: string): Instruction => {
  const parts = line.split(" ");
  if (parts[0] === "rect") {
    const [x, y] = parts[1].split("x").map((n) => parseInt(n));
    return [create, { x, y }];
  }

  const axis = parts[1] == "column" ? "C" : "R";
  const index = parseInt(parts[2].split("=")[1]);
  const n = parseInt(parts[4]);

  return [rotate, { axis, index, n }];
};

const solve = async () => {
  const instructions = (await getInputLines(import.meta.url, "real")).map(
    parseInstruction
  );

  let lcd = create2d(50, 6, false);

  for (const [instruction, args] of instructions) {
    lcd = instruction(lcd, args);
  }

  console.log(`Pixels illuminated: ${count2d(true, lcd)}`);

  // Pretty happy with this idea :)
  const mapping = new Map();
  mapping.set(true, "#").set(false, " ");
  console.log("Code Displayed:");
  print2d(lcd, mapping, 2);
};

if (import.meta.main) {
  solve();
}

export { solve };

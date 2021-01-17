import { getInputLines } from "../../aoc-helper.ts";
import { countUniqueChars } from "../../aoc-utils.ts";

const rotateColumns = (columns: string[]): string[] => {
  const rotated: string[] = [];

  for (const line of columns) {
    for (const [i, c] of Array.from(line).entries()) {
      if (rotated.length - 1 < i) {
        rotated[i] = "";
      }

      rotated[i] += c;
    }
  }

  return rotated;
};

const getCommonChar = (s: string, least = false): string => {
  const unique = countUniqueChars(s);

  if (least) return unique.sort()[0][1];
  return unique.sort().reverse()[0][1];
};

const solve = async () => {
  const cols = rotateColumns(await getInputLines(import.meta.url, "real"));

  console.log(`P1: ${cols.map((c) => getCommonChar(c)).join("")}`);
  console.log(`P2: ${cols.map((c) => getCommonChar(c, true)).join("")}`);
};

if (import.meta.main) {
  solve();
}

export { solve };

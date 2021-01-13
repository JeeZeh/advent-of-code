import { getInputLines } from "../../aoc-helper.ts";
import { chunkArray } from "../../aoc-utils.ts";

type Triangle = [number, number, number];
const re_number = new RegExp(/\d+/gi);

const parseSides = (line: string): Triangle => {
  const parts = (line.match(re_number) ?? []).map((x) => parseInt(x));

  return [parts[0], parts[1], parts[2]];
};

const isValidTriangle = (t: Triangle): boolean => {
  return (
      t[0] + t[1] > t[2] &&
      t[1] + t[2] > t[0] &&
      t[2] + t[0] > t[1] 
  );
};

const generateColumnTrianlges = (ts: Triangle[]): Triangle[] => {
  let colTriangles: Triangle[] = [];  

  for (let c = 0; c < ts[0].length; c++) {
    colTriangles.push([ts[0][c], ts[1][c], ts[2][c]]);
  }

  return colTriangles;
};

const solve = async () => {
  const lines = await getInputLines(import.meta.url, "real");

  const triangles: Triangle[] = lines.map(parseSides);

  console.log(
    `Valid triangles by row: ${triangles.filter(isValidTriangle).length}`
  );

  const validColumnTriangles = chunkArray(triangles, 3)
    .map(generateColumnTrianlges)
    .flat()
    .filter(isValidTriangle);  

  console.log(`Valid trianlges by column: ${validColumnTriangles.length}`);
};

if (import.meta.main) {
  solve();
}

export { solve };

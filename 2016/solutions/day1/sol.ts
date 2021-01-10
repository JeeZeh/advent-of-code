import { getInputFile } from "../../aoc-helper.ts";

type Point = [number, number];

const addPoint = (p1: Point, p2: Point): Point => [
  p1[0] + p2[0],
  p1[1] + p2[1],
];

const multPoint = (p1: Point, coeff: number): Point => [
  p1[0] * coeff,
  p1[1] * coeff,
];

const rotate = (point: Point, angle: number): Point => {
  angle = -(Math.PI / 180) * angle;
  let px = point[0];
  let py = point[1];
  let qx = Math.cos(angle) * px - Math.sin(angle) * py;
  let qy = Math.sin(angle) * px + Math.cos(angle) * py;
  return [Math.round(qx), Math.round(qy)];
};

const solve = async () => {
  let history: Set<string> = new Set();
  let heading: Point = [0, 1];
  let position: Point = [0, 0];
  let bunnyHq: Point | undefined;

  const lines = await getInputFile(import.meta.url, "real");
  const instructions = lines[0].split(", ");

  for (const ins of instructions) {
    const rotationDirection = ins[0] == "L" ? -1 : 1;
    const steps = parseInt(ins.slice(1));

    heading = rotate(heading, 90 * rotationDirection);

    for (let i = 0; i < steps; i++) {
      position = addPoint(position, heading);
      const posString = position.join(",");

      if (bunnyHq == null && history.has(posString)) {
        bunnyHq = position;
      }
      history.add(posString);
    }
  }

  console.log(
    `Distance from start: ${Math.abs(position[0]) + Math.abs(position[1])}`
  );
  if (bunnyHq != null) {
    console.log(
      `Distance Easter Bunny HQ: ${Math.abs(bunnyHq[0]) + Math.abs(bunnyHq[1])}`
    );
  }
};

export { solve };

if (import.meta.main) {
  solve();
}

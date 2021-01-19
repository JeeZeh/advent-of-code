export class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  add(p: Point) {
    return new Point(this.x + p.x, this.y + p.y);
  }

  scale(c: number) {
    return new Point(this.x * c, this.y * c);
  }
}

export function chunkArray(arr: any[], size: number) {
  return Array.from({ length: Math.ceil(arr.length / size) }, (v, i) =>
    arr.slice(i * size, i * size + size)
  );
}

export function count<T>(e: T, i: Iterable<T>): number {
  let c = 0;

  for (const el of i) {
    if (el === e) {
      c++;
    }
  }

  return c;
}

export function count2d<T>(e: T, i: T[][]): number {
  let c = 0;

  for (const row of i) {
    c += count(e, row);
  }

  return c;
}

export function stringMult(char: string, n: number): string {
  const row = [];

  for (let i = 0; i < n; i++) {
    row.push(char);
  }

  return row.join("");
}

export function print2d<T>(a: T[][], mapping: Map<T, string>, pad?: number) {
  let repr: string[] = [];

  if (pad != null) {
    repr.push(stringMult(" ", a.length));
  }

  for (const row of a) {
    let reprRow: (T | string)[] = [];
    for (const e of row) {
      reprRow.push(mapping.get(e) ?? e);
    }

    const rowPad = pad != null ? stringMult(" ", pad) : "";

    repr.push(rowPad + reprRow.join("") + rowPad);
  }

  if (pad != null) {
    repr.push(stringMult(" ", a.length));
  }

  console.log(repr.join("\n"));
}

export function tryParseInt(toCheck: string) {
  try {
    return parseInt(toCheck);
  } catch {
    return null;
  }
}

export function countUniqueChars(s: string): [number, string][] {
  return Array.from(new Set<string>(s)).map(
    (char) => [count(char, s), char] as [number, string]
  );
}

export function copy2d<T>(a: T[][]): T[][] {
  return a.map((a_) => a_.slice());
}

export function create2d<T>(width: number, height: number, init: T): T[][] {
  let array: T[][] = [];

  for (let i = 0; i < height; i++) {
    let row: T[] = [];
    for (let j = 0; j < width; j++) {
      row.push(init);
    }
    array.push(row);
  }

  return array;
}

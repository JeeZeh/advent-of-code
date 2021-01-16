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

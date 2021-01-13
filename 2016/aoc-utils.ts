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

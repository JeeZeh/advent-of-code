import { assert } from "https://deno.land/std@0.83.0/_util/assert.ts";
import { getInputLines } from "../../aoc-helper.ts";
import { float, int, Point } from "../../aoc-utils.ts";

class File {
  pos: Point; // (0,0)
  filename: string; // /dev/grid/node-x0-y0
  size: number; // 92
  used: number; // 72
  free: number; // 20
  alloc: number; // 78

  constructor(
    pos: Point,
    filename: string,
    size: number,
    used: number,
    free: number,
    alloc: number
  ) {
    this.pos = pos;
    this.filename = filename;
    this.size = size;
    this.used = used;
    this.free = free;
    this.alloc = alloc;
  }

  canPair(b: File) {
    return this.used > 0 && this != b && this.used <= b.free;
  }
}

const re_Spaces = /\s+/gi;
const re_Pos = /(?!(x|y))\d+(?=(\-|\s|$))/gi;

const parseFile = (f: string): File => {
  f = f.replace(re_Spaces, "|");
  const [filename, size, used, free, alloc] = f.split("|");

  const posXY = filename.match(re_Pos)?.map((x) => int(x));
  let pos: Point | undefined;

  if (posXY?.length == 2) {
    pos = new Point(posXY[0], posXY[1]);
  }

  if (pos != null) {
    return new File(
      pos,
      filename,
      int(size.slice(0, size.length - 1)),
      int(used.slice(0, used.length - 1)),
      int(free.slice(0, free.length - 1)),
      int(alloc.slice(0, alloc.length - 1))
    );
  } else {
    throw new Error("File borked");
  }
};

const solve = async () => {
  const files = (await getInputLines(import.meta.url, "real"))
    .slice(2)
    .map(parseFile);

  let pairs = new Map<File, File>();
  for (const a of files) {
    for (const b of files) {
      if (a.canPair(b) && pairs.get(b) != a) {
        pairs.set(a, b);
      }
    }
  }

  console.log(pairs.size);
};

const test = () => {
  const a = parseFile("/dev/grid/node-x22-y9    90T   69T    21T   76%");
  const b = parseFile("/dev/grid/node-x22-y9    90T   69T    21T   76%");
  const c = parseFile("/dev/grid/node-x22-y9    90T   21T    69T   24%");

  assert(a != null);
  assert(b != null);
  assert(c != null);

  assert(a == a);
  assert(a != b);

  assert(a.canPair(b) == false);
  assert(a.canPair(c) == true);
  console.info("Tests passed");
};

if (import.meta.main) {
  test();
  solve();
}

export { solve };

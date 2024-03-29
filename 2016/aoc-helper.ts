import * as path from "https://deno.land/std@0.83.0/path/mod.ts";
import { readLines } from "https://deno.land/std@0.83.0/io/bufio.ts";

const getInputLines = async (callerUrl: string, type: string) => {
  let callerPath = new URL("", callerUrl).pathname.split("/");
  let callerDir = callerPath.slice(1, callerPath.length - 1).join("/");
  const filename = path.join(callerDir, `${type}.txt`);
  const file = await Deno.open(filename);

  const out: string[] = [];

  for await (const x of readLines(file)) {
    out.push(x);
  }

  return out;
};

if (import.meta.main && !!Deno.args.length) {
  for await (const arg of Deno.args) {
    import(`./solutions/day${arg}/sol.ts`).then(async (solved) => {
      console.log(`--- Running Day ${arg} ---`);
      await solved.solve();
    });
  }
}

export { getInputLines };

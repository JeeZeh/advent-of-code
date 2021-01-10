import * as path from "https://deno.land/std@0.83.0/path/mod.ts";
import { readLines } from "https://deno.land/std@0.83.0/io/bufio.ts";

const getInputFile = async (callerUrl: string, type: string) => {
  let callerPath = new URL("", callerUrl).pathname.split("/");
  let callerDir = callerPath.slice(1, callerPath.length - 1).join("/");
  const filename = path.join(callerDir, `${type}.txt`);
  const file =  await Deno.open(filename);

  const out: string[] = [];

  for await(const x of readLines(file)) {
    out.push(x);
  }

  return out;
};


if (import.meta.main && !!Deno.args.length) {
  import("./solutions/solutions.ts").then(async solved => {
    for await (const arg of Deno.args) {
      console.log(`--- Running Day ${arg} ---`);
      await solved.solutions[`d${arg}`]();
    };
  })
}

export { getInputFile };

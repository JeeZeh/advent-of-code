import * as path from "https://deno.land/std@0.83.0/path/mod.ts";
import { solutions } from "./solutions/solutions.ts";

const getInputFile = async (callerUrl: string, type: string) => {
  let callerPath = new URL("", callerUrl).pathname.split("/");
  let callerDir = callerPath.slice(1, callerPath.length - 1).join("/");
  const filename = path.join(callerDir, `${type}.txt`);
  return await Deno.open(filename); 
};

const runner = async (day: string) => {
  console.log(`--- Running Day ${day} ---`);
  solutions[`d${day}`]();
};

if (import.meta.main && !!Deno.args.length) {
  Deno.args.forEach(async (arg) => {
    await runner(arg);
  });
}

export { getInputFile };

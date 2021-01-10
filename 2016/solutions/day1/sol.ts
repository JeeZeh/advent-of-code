import { readLines } from "https://deno.land/std@0.83.0/io/mod.ts";
import { getInputFile } from "../../aoc-helper.ts";

const solve = async () => {
  const input = await getInputFile(import.meta.url, "test");  
  for await (const line of readLines(input)) {
    console.log(line);
  }
};


export { solve };

if (import.meta.main) {
  solve();
}
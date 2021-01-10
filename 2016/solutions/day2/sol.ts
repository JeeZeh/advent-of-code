import { readLines } from "https://deno.land/std@0.83.0/io/mod.ts";
import { getInputFile } from "../../aoc-helper.ts";

const fact = (n: number): number => {
  if (n === 0 || n === 1) return 1;

  return n * fact(n-1);
}

const solve = async () => {
  const input = await getInputFile(import.meta.url, "test");  
  for await (const line of readLines(input)) {
    console.log(line);
  }

  console.log(`The factorial of 30 is ${fact(30)}`)
};


export { solve };

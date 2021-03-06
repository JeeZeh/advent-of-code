import { getInputLines } from "../../aoc-helper.ts";
const solve = async () => {
  const compressedText = (await getInputLines(import.meta.url, "hard"))[0];

  let decompressed = 0;

  let pointer = 0;
  while (pointer < compressedText.length) {
    const char = compressedText[pointer];

    if (char === "(") {
      const substart = pointer + 1;
      while (compressedText[pointer] !== ")") pointer++;
      const subparts = compressedText.slice(substart, pointer).split("x");
      let [count, repeat] = [parseInt(subparts[0]), parseInt(subparts[1])];
      pointer += count;
      decompressed += count * repeat;
    } else {
      decompressed += 1;
    }

    pointer++;
  }

  console.log(decompressed);
  return decompressed;
};

if (import.meta.main) {
  solve();
}

export { solve };

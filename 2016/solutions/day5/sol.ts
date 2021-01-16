import { tryParseInt } from "../../aoc-utils.ts";
import md5 from "./md5.ts";

function* getPaddedHash(
  padding: string,
  prefix: string
): Generator<[string, number], void, unknown> {
  let index = 0;
  let hash = "";

  while (true) {
    hash = md5(prefix + index);
    if (hash.startsWith(padding)) {
      yield [hash, index];
    }
    index++;
  }
}

const partOne = (doorId: string): string => {
  let hashIterator = getPaddedHash("00000", doorId);
  let password = "";

  while (password.length < 8) {
    let nextHash = hashIterator.next().value;

    if (Array.isArray(nextHash)) {
      password += nextHash[0].charAt(5);
      console.log(
        `Found next character '${nextHash[0].charAt(5)}' at index ${
          nextHash[1]
        }`
      );
    }
  }

  return password;
};

const partTwo = (doorId: string): string => {
  let hashIterator = getPaddedHash("00000", doorId);
  let passwordMap = new Map<number, string>();

  while (passwordMap.size < 8) {
    let nextHash = hashIterator.next().value;

    if (Array.isArray(nextHash)) {
      const pos = tryParseInt(nextHash[0].charAt(5));
      const char = nextHash[0].charAt(6);

      if (pos !== null && pos < 8 && !passwordMap.has(pos)) {
        passwordMap.set(pos, char);
        console.log(
          `Found next character '${char}' for position '${pos}' at index ${nextHash[1]}`
        );
      }
    }
  }

  return Array.from(passwordMap)
    .sort()
    .map((x) => x[1])
    .join("");
};

const solve = async () => {
  const doorId = "reyedfim";

  console.log("** Cracking door #1 **");
  console.log(`The password for the first door is '${partOne(doorId)}'`);

  console.log("\n** Cracking door #2 **");
  console.log(`The password for the second door is '${partTwo(doorId)}'`);
};

if (import.meta.main) {
  solve();
}

export { solve };

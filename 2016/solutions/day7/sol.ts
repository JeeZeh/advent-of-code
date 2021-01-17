import { getInputLines } from "../../aoc-helper.ts";

const nonHypernetRegExp = /(?<=(]|^))\w+/g;
const hypernetRegExp = /(?<=\[)\w+/g;

const hasAbba = (part: string): boolean => {
  let [start, end] = [0, 3];

  while (end < part.length) {
    const [a, b, c, d] = part.substring(start, end + 1);
    if (a !== b && a == d && b == c) return true;

    start++;
    end++;
  }

  return false;
};

const findAbas = (part: string): string[] => {
  let matches: string[] = [];

  let [start, end] = [0, 2];

  while (end < part.length) {
    const [a, b, c] = part.substring(start, end + 1);
    if (a !== b && a == c) matches.push(`${a}${b}${c}`);

    start++;
    end++;
  }
  return matches;
};

class IPv7 {
  nonHypernet: string[];
  hypernet: string[];

  constructor(nonHypernet: string[], hypernet: string[]) {
    this.nonHypernet = nonHypernet;
    this.hypernet = hypernet;
  }

  supportsTLS(): boolean {
    return (
      this.hypernet.find(hasAbba) == null &&
      this.nonHypernet.find(hasAbba) != null
    );
  }

  supportsSSL(): boolean {
    let nonHypernetABAs = new Set<string>();

    this.nonHypernet
      .map((h) => findAbas(h))
      .forEach((m) => (nonHypernetABAs = new Set([...nonHypernetABAs, ...m])));


    for (const matchArray of this.hypernet.map((h) => findAbas(h))) {
      for (const match of matchArray) {
        if (nonHypernetABAs.has(`${match[1]}${match[0]}${match[1]}`))
          return true;
      }
    }

    return false;
  }
}

const parseIp = (ip: string): IPv7 => {
  const nonHypernet = ip.match(nonHypernetRegExp) ?? [];
  const hypernet = ip.match(hypernetRegExp) ?? [];
  return new IPv7(nonHypernet, hypernet);
};

const solve = async () => {
  const ips = (await getInputLines(import.meta.url, "real")).map(parseIp);

  console.log(`IPs with TLS: ${ips.filter((ip) => ip.supportsTLS()).length}`);
  console.log(`IPs with SSL: ${ips.filter((ip) => ip.supportsSSL()).length}`);
};

if (import.meta.main) {
  solve();
}

export { solve };

import { getInputLines } from "../../aoc-helper.ts";
import { count } from "../../aoc-utils.ts";

type AGH = [number, string];

class Room {
  encName: string;
  sectorId: number;
  checksum: string;

  constructor(encName: string, sectorId: number, checksum: string) {
    this.encName = encName;
    this.sectorId = sectorId;
    this.checksum = checksum;
  }
}

const parseRoom = (room: string) => {
  const parts = room.split("-");

  let endPart = parts[parts.length - 1].split("[");
  let checksum = endPart[1].slice(0, endPart[1].length - 1);
  let sectorId = parseInt(endPart[0]);

  return new Room(
    parts.slice(0, parts.length - 1).join("-"),
    sectorId,
    checksum
  );
};

const sortAlphaNum = (a: [number, string], b: [number, string]): number => {
  if (a[0] < b[0]) return -1;

  if (a[0] > b[0]) return 1;

  if (a[1] > b[1]) return -1;

  if (a[1] < b[1]) return 1;

  return 0;
};

const generateChecksum = (room: Room) => {
  return Array.from(new Set<string>(room.encName.replaceAll("-", "")))
    .map((char) => [count(char, room.encName), char] as AGH)
    .sort(sortAlphaNum)
    .reverse()
    .map((x: AGH) => x[1])
    .slice(0, 5)
    .join("");
};

const solve = async () => {
  const validRooms = (await getInputLines(import.meta.url, "real"))
    .map(parseRoom)
    .filter((room) => room.checksum === generateChecksum(room));

  const sectorSums = validRooms
    .map((x) => x.sectorId)
    .reduce((acc, curr) => acc + curr);

  console.log(`Sum of real room sector IDs: ${sectorSums}`);
};

if (import.meta.main) {
  solve();
}

export { solve };

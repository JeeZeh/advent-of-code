import { getInputLines } from "../../aoc-helper.ts";
import { countUniqueChars } from "../../aoc-utils.ts";

const alpha = "abcdefghijklmnopqrstuvwxyz";
class Room {
  encName: string;
  sectorId: number;
  checksum: string;
  decName: string;

  constructor(encName: string, sectorId: number, checksum: string) {
    this.encName = encName;
    this.sectorId = sectorId;
    this.checksum = checksum;
    this.decName = this.decodeName(encName, sectorId);
  }

  decodeName = (encName: string, sectorId: number) => {
    let decName = "";

    for (const char of encName) {
      if (char === "-") {
        decName += " ";
      } else {
        const shift = (alpha.indexOf(char) + sectorId) % alpha.length;
        decName += alpha.charAt(shift);
      }
    }

    return decName;
  };
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
  return countUniqueChars(room.encName.replaceAll("-", ""))
    .sort(sortAlphaNum)
    .reverse()
    .map((x) => x[1])
    .slice(0, 5)
    .join("");
};

const solve = async () => {
  const rooms = (await getInputLines(import.meta.url, "real")).map(parseRoom);

  const realRooms = rooms.filter(
    (room) => room.checksum === generateChecksum(room)
  );

  const sectorSums = realRooms
    .map((x) => x.sectorId)
    .reduce((acc, curr) => acc + curr);

  console.log(`Sum of real room sector IDs: ${sectorSums}`);

  const roomSearchName = "northpole";
  const northPollRoom = realRooms.find((v) =>
    v.decName.includes(roomSearchName)
  );

  if (northPollRoom) {
    console.log(
      `Objects may be stored in "${northPollRoom.decName}", sector ID "${northPollRoom.sectorId}"`
    );
  } else {
    console.log(`Could not locate room matching name "${roomSearchName}"`);
  }

};

if (import.meta.main) {
  solve();
}

export { solve };

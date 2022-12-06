import fs from "fs";

const stackKeys = [1, 2, 3, 4, 5, 6, 7, 8, 9] as const;

type StackKey = typeof stackKeys[number];

const stacks: Record<StackKey, string[]> = {
  1: ["N", "V", "C", "S"],
  2: ["S", "N", "H", "J", "M", "Z"],
  3: ["D", "N", "J", "G", "T", "C", "M"],
  4: ["M", "R", "W", "J", "F", "D", "T"],
  5: ["H", "F", "P"],
  6: ["J", "H", "Z", "T", "C"],
  7: ["Z", "L", "S", "F", "Q", "R", "P", "D"],
  8: ["W", "P", "F", "D", "H", "L", "S", "C"],
  9: ["Z", "G", "N", "F", "P", "M", "S", "D"],
};

const rawInput = fs.readFileSync("input.txt").toString();

const splitInput = rawInput.split("\n").filter((entry) => entry.length > 0);

function validStackNumber(input: number): input is StackKey {
  return input > 0 && input < 10;
}

for (const instruction of splitInput) {
  const [_move, amountString, _from, fromString, _to, toString] =
    instruction.split(" ");

  const [amount, from, to] = [amountString, fromString, toString].map(Number);

  if (!validStackNumber(from) || !validStackNumber(to)) {
    throw new Error("Encountered input entry with invalid stack number");
  }

  // const removedPartOne = stacks[from].splice(0, amount).reverse();
  const removedPartTwo = stacks[from].splice(0, amount);

  // stacks[to] = removedPartOne.concat(stacks[to]);
  stacks[to] = removedPartTwo.concat(stacks[to]);

  console.info(`moved ${removedPartTwo} from stack ${from} to stack ${to}`);
}

console.info(
  "Top crates for every stack:",
  Object.values(stacks)
    .map((s) => s[0])
    .join("")
);

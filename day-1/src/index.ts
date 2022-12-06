import fs from "fs";

fs.readFile("./input.txt", (error, data) => {
  if (error) {
    console.error(error);

    return;
  }

  const groups = data.toString().split("\n\n");

  const cleaned = groups.map((group) => {
    return group.split("\n").map((entry) => Number(entry));
  });

  const summed = cleaned.map((group) => {
    return group.reduce((previous, num) => previous + num, 0);
  });

  const sorted = summed.sort((a, b) => (a > b ? -1 : 1));

  // answer to part one = sorted[0] (69883)
  console.log("answer to part one:", sorted[0]);

  // part two:

  const [first, second, third] = sorted;

  console.log("answer to part two:", first + second + third);
});

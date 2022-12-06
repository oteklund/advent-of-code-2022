/**
 * day 2: rock, paper, scissors
 */

/**
 * Opponent's moves:
 * A: rock
 * B: paper
 * C: scissors
 *
 * My moves:
 * X: rock (value: 1)
 * Y: paper (value: 2)
 * Z: scissors (value: 3)
 *
 * A draw yields 3 points, winning yields 6
 */

import fs from "fs";

const Move = {
  rock: "rock",
  paper: "paper",
  scissors: "scissors",
} as const;

const OpponentMove = {
  A: Move.rock,
  B: Move.paper,
  C: Move.scissors,
} as const;

const PlayerMove = {
  X: Move.rock,
  Y: Move.paper,
  Z: Move.scissors,
} as const;

const ShapeValue = {
  rock: 1,
  paper: 2,
  scissors: 3,
} as const;

const OutcomeValue = {
  win: 6,
  loss: 0,
  draw: 3,
} as const;

const OutcomeCodeMap = {
  X: "loss",
  Y: "draw",
  Z: "win",
} as const;

type PairPartOne = readonly [
  keyof typeof OpponentMove,
  keyof typeof PlayerMove
];

type PairPartTwo = readonly [
  keyof typeof OpponentMove,
  keyof typeof OutcomeCodeMap
];

function getOutcome(
  theirMove: typeof Move[keyof typeof Move],
  ourMove: typeof Move[keyof typeof Move]
): keyof typeof OutcomeValue {
  switch (`${theirMove} ${ourMove}`) {
    case "rock rock":
      return "draw";
    case "rock paper":
      return "win";
    case "rock scissors":
      return "loss";

    case "paper rock":
      return "loss";
    case "paper paper":
      return "draw";
    case "paper scissors":
      return "win";

    case "scissors rock":
      return "win";
    case "scissors paper":
      return "loss";
    case "scissors scissors":
      return "draw";
  }

  throw new Error("Failed to get outcome");
}

function calculateScorePartOne([theirs, ours]: PairPartOne): number {
  const theirMove = OpponentMove[theirs];
  const ourMove = PlayerMove[ours];

  const outcome = getOutcome(theirMove, ourMove);

  return OutcomeValue[outcome] + ShapeValue[ourMove];
}

function getOurMove(
  theirMove: typeof Move[keyof typeof Move],
  intendedOutcome: keyof typeof OutcomeValue
): typeof Move[keyof typeof Move] {
  switch (`${theirMove} ${intendedOutcome}`) {
    case "rock loss":
      return "scissors";
    case "rock draw":
      return "rock";
    case "rock win":
      return "paper";

    case "paper loss":
      return "rock";
    case "paper draw":
      return "paper";
    case "paper win":
      return "scissors";

    case "scissors loss":
      return "paper";
    case "scissors draw":
      return "scissors";
    case "scissors win":
      return "rock";
  }

  throw new Error("Failed to get our move");
}

function calculateScorePartTwo([theirs, outcomeCode]: PairPartTwo): number {
  const theirMove = OpponentMove[theirs];
  const outcome = OutcomeCodeMap[outcomeCode];

  const ourMove = getOurMove(theirMove, outcome);

  return OutcomeValue[outcome] + ShapeValue[ourMove];
}

function validateParsed(input: unknown): input is PairPartOne[] {
  return (
    Array.isArray(input) &&
    input.every((pair) => {
      return (
        pair.length === 2 && /[ABC]/.test(pair[0]) && /[XYZ]/.test(pair[1])
      );
    })
  );
}

fs.readFile("./input.txt", (error, data) => {
  if (error) {
    console.error(error);

    return;
  }

  const dataString = data.toString();

  const parsed: unknown = dataString
    .split("\n")
    .map((lineWithSpace) => lineWithSpace.split(" "))
    .filter((element) => element.length === 2);

  if (!validateParsed(parsed)) {
    throw new Error("failed to validate");
  }

  const totalPartOne = parsed.reduce((previous, pair) => {
    return previous + calculateScorePartOne(pair);
  }, 0);

  const totalPartTwo = parsed.reduce((previous, pair) => {
    return previous + calculateScorePartTwo(pair);
  }, 0);

  console.log("part one result:", totalPartOne); // 13675

  console.log("part two result:", totalPartTwo); // 14184
});

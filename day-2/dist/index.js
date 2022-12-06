"use strict";
/**
 * day 2: rock, paper, scissors
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
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
const fs_1 = __importDefault(require("fs"));
const Move = {
    rock: "rock",
    paper: "paper",
    scissors: "scissors",
};
const OpponentMove = {
    A: Move.rock,
    B: Move.paper,
    C: Move.scissors,
};
const PlayerMove = {
    X: Move.rock,
    Y: Move.paper,
    Z: Move.scissors,
};
const ShapeValue = {
    rock: 1,
    paper: 2,
    scissors: 3,
};
const OutcomeValue = {
    win: 6,
    loss: 0,
    draw: 3,
};
const OutcomeCodeMap = {
    X: "loss",
    Y: "draw",
    Z: "win",
};
function getOutcome(theirMove, ourMove) {
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
function calculateScorePartOne([theirs, ours]) {
    const theirMove = OpponentMove[theirs];
    const ourMove = PlayerMove[ours];
    const outcome = getOutcome(theirMove, ourMove);
    return OutcomeValue[outcome] + ShapeValue[ourMove];
}
function getOurMove(theirMove, intendedOutcome) {
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
function calculateScorePartTwo([theirs, outcomeCode]) {
    const theirMove = OpponentMove[theirs];
    const outcome = OutcomeCodeMap[outcomeCode];
    const ourMove = getOurMove(theirMove, outcome);
    return OutcomeValue[outcome] + ShapeValue[ourMove];
}
function validateParsed(input) {
    return (Array.isArray(input) &&
        input.every((pair) => {
            return (pair.length === 2 && /[ABC]/.test(pair[0]) && /[XYZ]/.test(pair[1]));
        }));
}
fs_1.default.readFile("./input.txt", (error, data) => {
    if (error) {
        console.error(error);
        return;
    }
    const dataString = data.toString();
    const parsed = dataString
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

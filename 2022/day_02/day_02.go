package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	input, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(input)

	// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
	// If both players choose the same shape, the round instead ends in a draw.
	// Player one: A for Rock, B for Paper, and C for Scissors
	// Player two: X Rock, Y for Paper, and Z for Scissors
	// test output part 1 is 15 (8 + 1 + 6)
	// test output part 2 is 12 (4 + 1 + 7)

	totalScore := 0
	for scanner.Scan() {
		played := strings.Fields(scanner.Text())
		score := 0

		// PART 2:

		if (played[1] == "X") {
			// X means you need to lose,
			if (played[0] == "A") {
				played[1] = "Z"
			} else if (played[0] == "B") {
				played[1] = "X"
			} else if (played[0] == "C") {
				played[1] = "Y"
			}
		} else if (played[1] == "Y") {
			// Y means you need to end the round in a draw
			if (played[0] == "A") {
				played[1] = "X"
			} else if (played[0] == "B") {
				played[1] = "Y"
			} else if (played[0] == "C") {
				played[1] = "Z"
			}
		} else if (played[1] == "Z") {
			// Z means you need to win
			if (played[0] == "A") {
				played[1] = "Y"
			} else if (played[0] == "B") {
				played[1] = "Z"
			} else if (played[0] == "C") {
				played[1] = "X"
			}
		}

		// Player two: 1 for Rock, 2 for Paper, and 3 for Scissors

		if (played[1] == "X") {
			score += 1
		} else if (played[1] == "Y") {
			score += 2
		} else if (played[1] == "Z") {
			score += 3
		}

		// The score for the outcome of the round
		// 0 if you lost, 3 if the round was a draw, and 6 if you won.

		if (played[0] == "C" && played[1] == "X") {
			score += 6
		} else if (played[0] == "B" && played[1] == "Z") {
			score += 6
		} else if (played[0] == "A" && played[1] == "Y") {
			score += 6
		}  else if (played[0] == "A" && played[1] == "X") {
			score += 3
		} else if (played[0] == "B" && played[1] == "Y") {
			score += 3
		} else if (played[0] == "C" && played[1] == "Z") {
			score += 3
		}

		totalScore += score

		fmt.Println(played[0], played[1], score)
	}

	fmt.Println("Total score:", totalScore)
}

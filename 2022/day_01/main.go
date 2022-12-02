package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input, _ := os.ReadFile("input.txt")

	answerPartOne := partOne(string(input))
	fmt.Println("Most carried calories by one elf: ", answerPartOne)

	answerPartTwo := partTwo(string(input))
	fmt.Println("Carried calories by top three: ", answerPartTwo)
}

func partOne(input string) int {
	carriers := calculateCaloriesPerCarrier(input)
	sort.Sort(sort.Reverse(sort.IntSlice(carriers)))

	return carriers[0]
}

func partTwo(input string) int {
	carriers := calculateCaloriesPerCarrier(input)
	sort.Sort(sort.Reverse(sort.IntSlice(carriers)))

	return carriers[0] + carriers[1] + carriers[2]
}

func calculateCaloriesPerCarrier(input string) []int {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	carriers := []int{0}
	carrier := 0
	for _, line := range lines {
		if line == "" {
			carrier++
			carriers = append(carriers, 0)
			continue
		}

		calories, _ := strconv.Atoi(line)
		carriers[carrier] += calories
	}

	return carriers
}

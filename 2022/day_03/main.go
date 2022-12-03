package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	input, _ := os.ReadFile("input.txt")

	answerPartOne := partOne(string(input))
	fmt.Println("Sum of priorities: ", answerPartOne)

	answerPartTwo := partTwo(string(input))
	fmt.Println("Part two: ", answerPartTwo)
}

func partOne(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	total := 0
	for _, line := range lines {
		compartments := getCompartmentItems(line)
		sharedItem := getSharedItem(compartments[0], compartments[1], "")
		priority := getPriority(sharedItem)

		total += priority
	}

	return total
}

func partTwo(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	groups := [][]string{}
	group := []string{}
	count := 0
	for _, line := range lines {
		group = append(group, line)
		count++

		if (count == 3) {
			groups = append(groups, group)
			group = nil
			count = 0
		}
	}

	total := 0
	for _, group := range groups {
		sharedItem := getSharedItem(group[0], group[1], group[2])
		priority := getPriority(sharedItem)

		total += priority

		fmt.Println(group, sharedItem, priority)
	}

	return total
}

func getCompartmentItems(items string) []string {
	length := len(items)
	half := length / 2
	strings.Split(items, "")

    var compartments []string
    var stop int
    for i := 0; i < length; i += half {
        stop = i + half
        if stop > length {
            stop = length
        }
        compartments = append(compartments, items[i:stop])
    }

	if (len(compartments) != 2) {
		panic("invalid amount of compartments found")
	}

    return compartments
}

func getSharedItem(a string, b string, c string) string {
	for _, item := range a {
		if (strings.Contains(b, string(item))) {
			if (c == "") {
				return string(item)
			}

			if (c != "" && strings.Contains(c, string(item))) {
				return string(item)
			}
		}
	}

	panic("no shared item found")
}

func getPriority(item string) int {
	char := []rune(item)
	value := int(char[0])

	if (value >= 97) {
		return value - 96
	}

	if (value >= 65) {
		return value - 38
	}

	panic("unknown priority")
}

// The ASCII value of the lowercase alphabet is from 97 to 122. And, the ASCII value of the uppercase alphabet is from 65 to 90.

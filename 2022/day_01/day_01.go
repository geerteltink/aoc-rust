package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	input, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(input)

	elfs := []int{0}
	currentElf := 0

	for scanner.Scan() {
		calories, _ := strconv.Atoi(scanner.Text())

		if calories != 0 {
			elfs[currentElf] += calories
		} else {
			currentElf++
			elfs = append(elfs, 0)
		}
	}

	// Part one
	maxCalories := elfs[0]
	maxElf := 0
	for elf, v := range elfs {
		if v > maxCalories {
			maxCalories = v
			maxElf = elf
		}
	}
	maxElf++

	fmt.Println("Elf", maxElf, "is carrying the max calories:", maxCalories)

	// Part two
	sort.Sort(sort.Reverse(sort.IntSlice(elfs)))
	fmt.Println("Most carried calories by one elf", elfs[0])
	fmt.Println("Carried calories by top three", elfs[0]+elfs[1]+elfs[2])
}

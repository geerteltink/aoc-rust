package main

import (
	"fmt"
	"os"
	"reflect"
	"strings"
	"testing"
)

func TestPartOne(t *testing.T) {
	input, _ := os.ReadFile("input_test.txt")

	expected := 24000
	actual := partOne(string(input))

	if (expected != actual) {
		t.Errorf(
			"The expected %s is different than the actual %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}
}

func TestPartTwo(t *testing.T) {
	input, _ := os.ReadFile("input_test.txt")

	expected := 45000
	actual := partTwo(string(input))

	if (expected != actual) {
		t.Errorf(
			"The expected %s is different than the actual %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}
}

func TestCalculateCaloriesPerCarrier(t *testing.T) {
	input, _ := os.ReadFile("input_test.txt")

	expected := []int{6000,4000,11000,24000,10000}
	actual := calculateCaloriesPerCarrier(string(input))

	if (!reflect.DeepEqual(expected, actual)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}
}

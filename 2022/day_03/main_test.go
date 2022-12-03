package main

import (
	"fmt"
	"os"
	"strings"
	"testing"
	"reflect"
)

func TestPartOne(t *testing.T) {
	input, _ := os.ReadFile("input_test.txt")

	expected := 157
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

	expected := 70
	actual := partTwo(string(input))

	if (expected != actual) {
		t.Errorf(
			"The expected %s is different than the actual %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}
}

func TestGetCompartmentItems(t *testing.T) {
	expected := []string{"vJrwpWtwJgWr","hcsFMMfFFhFp"}
	actual := getCompartmentItems("vJrwpWtwJgWrhcsFMMfFFhFp")

	if (!reflect.DeepEqual(expected, actual)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}

	expected2 := []string{"jqHRNqRjqzjGDLGL","rsFMfFZSrLrFZsSL"}
	actual2 := getCompartmentItems("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")

	if (!reflect.DeepEqual(expected2, actual2)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected2), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual2), " ", ",", -1), "[]"),
		)
	}
}

func TestGetSharedItem(t *testing.T) {
	expected := "p"
	actual := getSharedItem("vJrwpWtwJgWr", "hcsFMMfFFhFp", "")

	if (!reflect.DeepEqual(expected, actual)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}

	expected2 := "L"
	actual2 := getSharedItem("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL", "")

	if (!reflect.DeepEqual(expected2, actual2)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected2), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual2), " ", ",", -1), "[]"),
		)
	}
}

func TestGetSharedItemBetweenThree(t *testing.T) {
	expected := "r"
	actual := getSharedItem("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg")

	if (!reflect.DeepEqual(expected, actual)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}
}

func TestGetPriority(t *testing.T) {
	expected := 16
	actual := getPriority("p")

	if (!reflect.DeepEqual(expected, actual)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual), " ", ",", -1), "[]"),
		)
	}

	expected2 := 38
	actual2 := getPriority("L")

	if (!reflect.DeepEqual(expected2, actual2)) {
		t.Errorf(
			"The expected array %s is different than the actual array %s",
			strings.Trim(strings.Replace(fmt.Sprint(expected2), " ", ",", -1), "[]"),
			strings.Trim(strings.Replace(fmt.Sprint(actual2), " ", ",", -1), "[]"),
		)
	}
}

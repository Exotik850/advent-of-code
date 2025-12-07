package main

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
)

func extractDigits(input string) (int, int) {
	// Define a regular expression pattern to match digits and words
	pattern := regexp.MustCompile(`\d|one|two|three|four|five|six|seven|eight|nine`)

	// Find all matches of digits and words in the input string
	matches := pattern.FindAllString(input, -1)

	// Create a slice to store the extracted digits
	first, _ := toInt(matches[0])
	last, _ := toInt(matches[len(matches)-1])
	return first, last
}

func toInt(input string) (int, error) {
	wordToDigit := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}
	if digit, ok := wordToDigit[input]; ok {
		return digit, nil
	}
	return strconv.Atoi(input)
}

func main() {
	file, err := os.Open("./test.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	sum := 0
	for scanner.Scan() {
		line := scanner.Text()

		// // Part 1
		// first := strings.IndexFunc(line, unicode.IsDigit)
		// last := strings.LastIndexFunc(line, unicode.IsDigit)
		// digitInt := int(line[first]-'0')*10 + int(line[last]-'0')
		// sum += digitInt

		// Part 2, find the first and last digit that is spelt out as a word
		first, last := extractDigits(line)
		// fmt.Printf("Extracted digits: %v\n", nums)
		// newNum := fmt.Sprintf("%d%d", nums[0], nums[len(nums)-1])
		// fmt.Printf("New number: %v\n", newNum)
		// num, err := strconv.Atoi(newNum)
		// if err != nil {
		// 	panic(err)
		// }
		calibration := first*10 + last
		println(calibration)
		sum += calibration
	}
	println(sum)
}

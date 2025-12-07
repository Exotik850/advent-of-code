package main

import (
	"bufio"
	"os"
	"slices"
	"strconv"
	"strings"
)

const (
	maxRed   = 12
	maxGreen = 13
	maxBlue  = 14
)

func main() {
	file, err := os.Open("./games.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	gameNum := 1
	sum := 0
	colorNames := []string{"red", "green", "blue"}
	for scanner.Scan() {
		line := scanner.Text()
		_, hands, _ := strings.Cut(line, ":")
		hands = strings.TrimSpace(hands)
		indHands := strings.FieldsFunc(hands, func(r rune) bool { return r == ';' })
		possible := true
	hands:
		for _, hand := range indHands {
			msg := strings.FieldsFunc(hand, func(r rune) bool { return r == ',' })
			for _, card := range msg {
				num, color, _ := strings.Cut(card, " ")
				colorInd := slices.Index(colorNames, color)
				if colorInd == -1 {
					continue
				}
				num = strings.TrimSpace(num)
				numInt, _ := strconv.Atoi(num)
				switch {
				case colorInd == 0 && numInt > maxRed:
					possible = false
				case colorInd == 1 && numInt > maxGreen:
					possible = false
				case colorInd == 2 && numInt > maxBlue:
					possible = false
				}
				if !possible {
					break hands
				}
			}
		}
		if possible {
			sum += gameNum
		}

		gameNum++
	}
	println(sum)
}

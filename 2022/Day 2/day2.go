package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

type Choice int

const (
	Rock Choice = iota
	Paper
	Scissors
)

func (c Choice) String() string {
	switch c {
	case Rock:
		return "Rock"
	case Paper:
		return "Paper"
	case Scissors:
		return "Scissors"
	default:
		return ""
	}
}

type Result int

const (
	Win Result = iota
	Lose
	Tie
)

func (r Result) Score() int {
	switch r {
	case Win:
		return 6
	case Lose:
		return 0
	case Tie:
		return 3
	default:
		return 0
	}
}

func (c Choice) Score() int {
	switch c {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	default:
		return 0
	}
}

func (c Choice) Compare(b Choice) Result {
	switch c {
	case Rock:
		switch b {
		case Rock:
			return Tie
		case Paper:
			return Lose
		case Scissors:
			return Win
		}
	case Paper:
		switch b {
		case Rock:
			return Win
		case Paper:
			return Tie
		case Scissors:
			return Lose
		}
	case Scissors:
		switch b {
		case Rock:
			return Lose
		case Paper:
			return Win
		case Scissors:
			return Tie
		}
	}
	return Lose
}

func (c Choice) Respond(r Result) Choice {
	switch c {
	case Rock:
		switch r {
		case Win:
			return Paper
		case Lose:
			return Scissors
		case Tie:
			return Rock
		}
	case Paper:
		switch r {
		case Win:
			return Scissors
		case Lose:
			return Rock
		case Tie:
			return Paper
		}
	case Scissors:
		switch r {
		case Win:
			return Rock
		case Lose:
			return Paper
		case Tie:
			return Scissors
		}
	}
	return Rock
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	start := time.Now()

	file, err := os.Open("strategy_guide.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	scanner := bufio.NewScanner(file)

	var score int
	for scanner.Scan() {
		line := scanner.Text()
		left, right, found := strings.Cut(line, " ")

		if !found {
			fmt.Println("Invalid line: ", line)
			continue
		}

		opp, e := ParseChoice(left)
		check(e)
		// mine, e := ParseChoice(right)
		result, e := ParseResult(right)
		check(e)
		// Part 1
		// score += mine.Compare(opp).Score()
		// score += mine.Score()

		// Part 2
		score += result.Score()
		score += opp.Respond(result).Score()
		fmt.Printf("score: %v\n", score)
	}

	fmt.Println(score)

	elapsed := time.Since(start)
	fmt.Printf("Time taken: %s\n", elapsed)
}

func ParseChoice(s string) (Choice, error) {
	switch s {
	case "A":
		return Rock, nil
	case "B":
		return Paper, nil
	case "C":
		return Scissors, nil
	case "X":
		return Rock, nil
	case "Y":
		return Paper, nil
	case "Z":
		return Scissors, nil
	default:
		return Rock, fmt.Errorf("Invalid choice: %s", s)
	}
}

func ParseResult(s string) (Result, error) {
	switch s {
	case "X":
		return Lose, nil
	case "Y":
		return Tie, nil
	case "Z":
		return Win, nil
	default:
		return Lose, fmt.Errorf("Invalid result: %s", s)
	}
}

package main

import (
	"bytes"
	"fmt"
	"os"
)

const START = 'S'
const END = 'E'

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func filter[T any](slice []T, f func(T) bool) []T {
	var n []T
	for _, e := range slice {
		if f(e) {
			n = append(n, e)
		}
	}
	return n
}

func IndexToCoord(index int, width int) (int, int) {
	return index % width, index / width
}

func main() {
	byteData, err := os.ReadFile("./input.txt")
	check(err)
	width := bytes.IndexByte(byteData, '\n')
	byteData = filter[byte](byteData, func(b byte) bool { return b != '\n' })

	height := len(byteData) / width
	START_POS_X, START_POS_Y := IndexToCoord(bytes.IndexByte(byteData, START), width)
	END_POS_X, END_POS_Y := IndexToCoord(bytes.IndexByte(byteData, END), width)

	fmt.Println("SX", START_POS_X, "SY", START_POS_Y)
	fmt.Println("EX", END_POS_X, "EY", END_POS_Y)
	fmt.Println("W", width, "H", height)
}

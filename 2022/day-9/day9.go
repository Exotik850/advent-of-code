package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

type Number interface {
	int16 | int32 | int8 | float32 | float64
}

type vec[T Number] struct {
	x T
	y T
}

func (v vec[T]) Add(rhs vec[T]) vec[T] {
	return vec[T]{x: v.x + rhs.x, y: v.y + rhs.y}
}

func (v *vec[T]) AddEq(rhs vec[T]) {
	v.x += rhs.x
	v.y += rhs.y
}

func (v vec[T]) Sub(rhs vec[T]) vec[T] {
	return vec[T]{x: v.x - rhs.x, y: v.y - rhs.y}
}

type rope struct {
	head      vec[int16]
	tail      vec[int16]
	positions map[vec[int16]]*int16
}

type Dir byte

const (
	Up    Dir = 'U'
	Down      = 'D'
	Left      = 'L'
	Right     = 'R'
)

func FromByte(b byte) Dir {
	switch b {
	case 'U':
		return Up
	case 'D':
		return Down
	case 'L':
		return Left
	case 'R':
		return Right
	}
	return Down
}

func (dir Dir) ToVec() vec[int16] {
	switch dir {
	case Up:
		return vec[int16]{x: 0, y: 1}
	case Down:
		return vec[int16]{x: 0, y: -1}
	case Left:
		return vec[int16]{x: -1, y: 0}
	case Right:
		return vec[int16]{x: 1, y: 0}
	}
	return vec[int16]{x: 0, y: 0}
}

func (self *rope) add(dir Dir) {
	dirVec := dir.ToVec()
	dist := self.tail.Sub(self.head)

	moved := false

	if dirVec.y == 0 && dist.x != dirVec.x {
		self.tail.x = self.head.x
		moved = true
	} else if dist.y != dirVec.y {
		self.tail.y = self.head.y
		moved = true
	}

	if moved {
		if p, ok := self.positions[self.tail]; ok {
			*p++
		} else {
			n := int16(1)
			self.positions[self.tail] = &n
		}
	}

	self.head.AddEq(dirVec)
}

func main() {
	var temp rope
	temp.positions = make(map[vec[int16]]*int16)

	file, err := os.Open("./input.txt")
	check(err)
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		pair := strings.Split(line, " ")
		if len(pair) != 2 {
			panic("Invalid Argument Length!")
		}
		dir := FromByte(pair[0][0])
		num, err := strconv.Atoi(pair[1])
		check(err)
		for i := 0; i < num; i++ {
			temp.add(dir)
		}
	}
	fmt.Println("Rope:",temp)
	fmt.Println("Len:",len(temp.positions))

	// fmt.Printf("data: %v\n", string(data))
}

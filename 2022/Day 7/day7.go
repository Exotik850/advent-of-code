package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type directory struct {
	name     string
	parent   *directory
	children map[string]*directory
	files    map[string]int
	size     int
}

func (d *directory) calculateSize() int {
	if d.size != 0 {
		return d.size
	}

	for _, childDir := range d.children {
		d.size += childDir.calculateSize()
	}

	for _, fileSize := range d.files {
		d.size += fileSize
	}

	return d.size
}

const (
	updateSize = 30000000
	diskSpace  = 70000000
)

func main() {
	file, err := os.Open("./commands.txt")
	if err != nil {
		panic(err)
	}
	scanner := bufio.NewScanner(file)
	root := &directory{name: "/", children: make(map[string]*directory), files: make(map[string]int)}
	currentDir := root

	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "$ cd") {
			dirName := line[5:]
			if dirName == "/" {
				currentDir = root
			} else if dirName == ".." {
				currentDir = currentDir.parent
			} else {
				currentDir = currentDir.children[dirName]
			}
		} else if strings.HasPrefix(line, "$ ls") {
			// Do nothing, as we parse the output of ls command in the subsequent lines
		} else if strings.HasPrefix(line, "dir") {
			dirName := line[4:]
			if _, ok := currentDir.children[dirName]; !ok {
				currentDir.children[dirName] = &directory{name: dirName, parent: currentDir, children: make(map[string]*directory), files: make(map[string]int)}
			}
		} else {
			parts := strings.Split(line, " ")
			fileSize, _ := strconv.Atoi(parts[0])
			fileName := parts[1]
			currentDir.files[fileName] = fileSize
		}
	}

	root.calculateSize()

	freeSpace := diskSpace - root.size
	fmt.Println("Free:", freeSpace)
	neededSpace := updateSize - freeSpace
	fmt.Println("Needed:", neededSpace)

	// Find the smallest directory that can be deleted to have enough
	// room for the update

	min := 4 * neededSpace
	minDir := root
	var dfs func(*directory)
	dfs = func(d *directory) {
		if d.size < min && d.size >= neededSpace {
			min = d.size
			minDir = root
		}
		for _, dir := range d.children {
			dfs(dir)
		}
	}
	dfs(root)

	fmt.Println(minDir.name, min)

}

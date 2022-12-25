package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/MellKam/advent_of_code/2022/day_12/golang/graph"
)

func main() {
	data, err := os.ReadFile("./2022/day_12/input.txt")
	if err != nil {
		panic(err)
	}

	g, startCoords, endCoords := FillGraph(string(data))

	startVertex := g.GetVertex(startCoords)
	if startVertex == nil {
		panic("startVertex is nil")
	}

	endVertex := g.GetVertex(endCoords)
	if endVertex == nil {
		panic("endVertex is nil")
	}

	shortestPath := graph.FindShortestPathToValue(endVertex, byte('a')-96)

	fmt.Println(shortestPath)
}

func FillGraph(data string) (graph.Graph, graph.Coords, graph.Coords) {
	var startCoords graph.Coords
	var endCoords graph.Coords

	lines := strings.Split(data, "\n")
	height := len(lines)
	width := len(lines[0])

	g := graph.Graph{Width: uint(width), Height: uint(height)}

	for y, line := range lines {
		for x, char := range line {
			var vertex graph.Vertex
			coords := graph.NewCoords(uint(x), uint(y))
			var value byte

			if char == 'S' {
				// "S" => start point (elevation 'a')
				value = byte('a')
				startCoords = coords
			} else if char == 'E' {
				// "E" => end point (elevation 'z')
				value = byte('z')
				endCoords = coords
			} else {
				value = byte(char)
			}

			vertex = graph.NewVertex(coords, value-96)
			err := g.AddVertex(vertex)
			if err != nil {
				panic(err)
			}
		}
	}

	nextVertex := g.Iter()

	for {
		vertex := nextVertex()

		if vertex == nil {
			break
		}

		coords := vertex.Key()

		left := g.GetVertex(graph.NewCoords(coords.X()-1, coords.Y()))
		right := g.GetVertex(graph.NewCoords(coords.X()+1, coords.Y()))
		top := g.GetVertex(graph.NewCoords(coords.X(), coords.Y()-1))
		bottom := g.GetVertex(graph.NewCoords(coords.X(), coords.Y()+1))

		ConnectVertexesByTarget(vertex, left)
		ConnectVertexesByTarget(vertex, right)
		ConnectVertexesByTarget(vertex, top)
		ConnectVertexesByTarget(vertex, bottom)
	}

	return g, startCoords, endCoords
}

func ConnectVertexesByTarget(v1 *graph.Vertex, v2 *graph.Vertex) {
	if v2 == nil {
		return
	}

	delta := int(v2.Value()) - int(v1.Value())

	// for part one with going up -> (v <= 1)
	// for part two with going down -> (v >= -1)
	if delta >= -1 {
		v1.AddEdge(v2)
	}
}

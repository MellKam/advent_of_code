package graph

import "fmt"

type Graph struct {
	veritces []*Vertex
	Height   uint
	Width    uint
}

func (g *Graph) AddVertex(vertex Vertex) error {
	if g.ContainsKey(&vertex.key) {
		return fmt.Errorf("vertex with key {%v} already exists", vertex.key.String())
	}

	g.veritces = append(g.veritces, &vertex)
	return nil
}

func (g *Graph) GetVertex(coords Coords) *Vertex {
	for _, v := range g.veritces {
		if v.key.Equal(&coords) {
			return v
		}
	}

	return nil
}

func (g *Graph) ContainsKey(coords *Coords) bool {
	for _, v := range g.veritces {
		if v.key.Equal(coords) {
			return true
		}
	}

	return false
}

func (g *Graph) String() string {
	s := "Graph {\n"
	for i, vertex := range g.veritces {
		if i > 0 {
			s += ",\n"
		}
		s += fmt.Sprintf("	%v", vertex.String())

	}
	return s + "\n}"
}

func (g *Graph) Iter() func() *Vertex {
	var index = 0
	var length = len(g.veritces)

	return func() *Vertex {
		if index >= length {
			return nil
		}

		v := g.veritces[index]
		index++
		return v
	}
}

func (g *Graph) Len() int {
	return len(g.veritces)
}

package graph

import "fmt"

type Vertex struct {
	key       Coords
	value     byte
	adjacents []*Vertex
}

func NewVertex(key Coords, value byte) Vertex {
	return Vertex{key: key, value: value}
}

func (v *Vertex) AddEdge(v2 *Vertex) {
	v.adjacents = append(v.adjacents, v2)
}

func (v *Vertex) Key() Coords {
	return v.key
}

func (v *Vertex) Value() byte {
	return v.value
}

func (v *Vertex) String() string {
	s := fmt.Sprintf("Vertex[%v] {", v.key.String())
	for i, vertex := range v.adjacents {
		if i > 0 {
			s += ", "
		}
		s += fmt.Sprintf("{%v}", vertex.key.String())
	}
	return s + "}"
}

func (v *Vertex) Iter() func() *Vertex {
	var index = 0
	var length = len(v.adjacents)

	return func() *Vertex {
		if index >= length {
			return nil
		}

		v := v.adjacents[index]
		index++
		return v
	}
}

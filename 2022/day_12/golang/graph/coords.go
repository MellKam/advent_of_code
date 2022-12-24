package graph

import "fmt"

type Coords struct {
	x uint
	y uint
}

func (c *Coords) Equal(c2 *Coords) bool {
	return c.x == c2.x && c.y == c2.y
}

func (c *Coords) String() string {
	return fmt.Sprintf("x: %d, y: %d", c.x, c.y)
}

func NewCoords(x uint, y uint) Coords {
	return Coords{x: x, y: y}
}

func (c *Coords) X() uint {
	return c.x
}

func (c *Coords) Y() uint {
	return c.y
}

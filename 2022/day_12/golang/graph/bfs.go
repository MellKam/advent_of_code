package graph

type BFSNode struct {
	vertex   *Vertex
	distance int
}

func GetShortestPath(startVertex *Vertex, endCoords Coords) int {
	visited := make(map[Coords]bool)
	queue := make([]BFSNode, 0)

	queue = append(queue, BFSNode{vertex: startVertex, distance: 0})
	visited[startVertex.Key()] = true

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		key := node.vertex.Key()
		if endCoords.Equal(&key) {
			return node.distance
		}

		nextNeighbour := node.vertex.Iter()
		for {
			neighbour := nextNeighbour()
			if neighbour == nil {
				break
			}

			key = neighbour.Key()
			if !visited[key] {
				queue = append(queue, BFSNode{vertex: neighbour, distance: node.distance + 1})
				visited[key] = true
			}
		}
	}

	return -1
}

func FindShortestPathToValue(startVertex *Vertex, targetValue byte) int {
	visited := make(map[Coords]bool)
	queue := make([]BFSNode, 0)

	queue = append(queue, BFSNode{vertex: startVertex, distance: 0})
	visited[startVertex.Key()] = true

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		if node.vertex.value == targetValue {
			return node.distance
		}

		nextNeighbour := node.vertex.Iter()
		for {
			neighbour := nextNeighbour()
			if neighbour == nil {
				break
			}

			key := neighbour.Key()

			if !visited[key] {
				queue = append(queue, BFSNode{vertex: neighbour, distance: node.distance + 1})
				visited[key] = true
			}
		}
	}

	return -1
}

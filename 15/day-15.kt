package some.test

import java.io.File
import java.util.PriorityQueue

data class Point(val x: Int, val y: Int)

data class PathCost(val point: Point, val cost: Int) : Comparable<PathCost> {
	public override fun compareTo(other: PathCost): Int =
		cost - other.cost
}

class Chiton(var relativePath: String) {
	private var input = File(System.getProperty("user.dir") + "/../" + relativePath)
		.readLines()
		.map { it.map { Character.getNumericValue(it) } }

	private fun getNeighbors(p: Point): List<Point> =
		listOf(Pair(-1, 0), Pair(1, 0), Pair(0, -1), Pair(0, 1))
			.map { Point(p.x + it.first, p.y + it.second) }
			.filter { it.x >= 0 && it.x < input.size && it.y >= 0 && it.y < input.size }

	fun topLeft(): Point =
		Point(0, 0)

	fun bottomRight(): Point =
		Point(input.size - 1, input.first().size - 1)

	fun dijkstra(start: Point, end: Point): Int {
		val costs = HashMap<Point, Int>()
		val heap = PriorityQueue<PathCost>()
		heap.add(PathCost(start, 0))

		while (true) {
			if (heap.peek() == null)
				break

			val pos = heap.poll()

			if (pos.point == end)
				break

			if (pos.cost <= costs.getOrDefault(pos.point, Int.MAX_VALUE))
				getNeighbors(pos.point)
					.map { PathCost(it, input[it.x][it.y] + pos.cost) }
					.filter { it.cost < costs.getOrDefault(it.point, Int.MAX_VALUE) }
					.forEach {
						heap.add(it)
						costs.put(it.point, it.cost)
					}
		}

		return costs.getOrDefault(end, Int.MAX_VALUE)
	}
}

fun main(args: Array<String>) {
	val chiton = Chiton(args[0])

	println(chiton.dijkstra(chiton.topLeft(), chiton.bottomRight()))
}

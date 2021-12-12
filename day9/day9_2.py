# Find the "low" points in the map, and sum their risk score (1 + height of the point)
# A point is "low" is it's the lowest among its neighbors (diagonal don't count)

# For part 2, compute the size of the 3 largest "baisins" (group of points in the map,
# different from 9, which "flow" to the same low point) and multiply their size
# To compute all the baisins, we use the flood-fill algorithm starting from the low points

import numpy as np
from collections import deque

def get_neighbors(map, y, x):
    neighbors = []
    if y - 1 >= 0 and map[y - 1, x] != 9:
        neighbors.append((y - 1, x))
    if x - 1 >= 0 and map[y, x - 1] != 9:
        neighbors.append((y, x - 1))
    if x + 1 < map.shape[1] and map[y, x + 1] != 9:
        neighbors.append((y, x + 1))
    if y + 1 < map.shape[0] and map[y + 1, x] != 9:
        neighbors.append((y + 1, x))
    return neighbors

def get_min_neighbor(map, y, x):
    return min([map[y,x] for (y,x) in get_neighbors(map, y, x)] + [9])

# Iterative flood-fill algorithm using deque as data-structure
def get_baisin_size(map, y, x):
    q = deque()
    visited = np.zeros(map.shape, dtype=bool)
    
    area = 0

    q.append((y,x))
    while q:
        y, x = q.popleft()
        if not visited[y,x]:
            neighbors = get_neighbors(map, y, x)
            for (yn, xn) in neighbors:
                if not visited[yn, xn]:
                    q.append((yn, xn))
            visited[y,x] = True

            area += 1
    return area


# Read input
input = np.genfromtxt('inputs.txt', dtype='i', delimiter=1)
input = np.pad(input, (1,1), constant_values=(9,9))

# Find low points
low_points = [(y,x) for x in range(0,input.shape[1]) for y in range(0,input.shape[0]) if input[y,x] < get_min_neighbor(input, y, x)]

# Perform flood-fill on all low points
baisin_sizes = [get_baisin_size(input, y, x) for (y,x) in low_points]
_1, _2, _3, *_ = sorted(baisin_sizes, reverse=True)

print(_1 * _2 * _3)
# Find the "low" points in the map, and sum their risk score (1 + height of the point)
# A point is "low" is it's the lowest among its neighbors (diagonal don't count)

import numpy as np

def get_min_neighbor(map, y, x):
    neighbors = []
    if y - 1 >= 0:
        neighbors.append(map[y - 1, x])
    if x - 1 >= 0:
        neighbors.append(map[y, x - 1])
    if x + 1 < map.shape[1]:
        neighbors.append(map[y, x + 1])
    if y + 1 < map.shape[0]:
        neighbors.append(map[y + 1, x])
    return min(neighbors)

# Read input
input = np.genfromtxt('inputs.txt', dtype='i', delimiter=1)

rank = 0
for y in range(0, input.shape[0]):
    for x in range(0, input.shape[1]):
        if input[y,x] < get_min_neighbor(input, y,x):
            rank += input[y,x] + 1

print(rank)
import sys
import numpy as np
from queue import PriorityQueue

def solve(matrix):
    # Initialize distances to infinite
    D = np.full(matrix.shape, sys.maxsize, dtype=int)
    D[0,0] = 0

    # Only horizontal and vertical, no diagonal movements
    deltas = [(0, 1), (1, 0), (-1, 0), (0, -1)]
    height, width = matrix.shape
    
    pq = PriorityQueue()
    pq.put((0, (0,0)))

    while not pq.empty():
        (d, (y,x)) = pq.get()
        for (dy, dx) in deltas:
            xx = x + dx
            yy = y + dy
            if 0 <= yy < height and  0 <= xx < width:
                new_d = d + int(matrix[yy,xx])
                # Update distance and add coord to queue if the distance is updated
                if new_d < D[yy,xx]:
                    D[yy,xx] = new_d
                    pq.put((new_d, (yy,xx)))
    
    return D[-1,-1]

def make_big_cave(cave):
    def wrap(element):
        return element % 9 if element % 9 > 0 else element
    vectorized_wrap = np.vectorize(wrap)
    
    h, w = cave.shape
    big_cave = np.zeros((h*5, w*5), dtype='i')
    
    for y in range(5):
        for x in range(5):
            new_mat = vectorized_wrap(np.copy(cave) + y + x)
            big_cave[h*y: h * (y+1), w*x : w*(x+1)] = new_mat
    return big_cave
            
cave = np.genfromtxt('input.txt', dtype='i',delimiter=1)
print("Part 1: " + str(solve(cave)))
print("Part 2: " + str(solve(make_big_cave(cave))))
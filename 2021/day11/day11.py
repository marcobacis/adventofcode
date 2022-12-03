import numpy as np

def flash(input, y, x):
    height, width = input.shape
    neighbors = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1), (1,-1), (1,0),(1,1)] # (y,x)
    valid_neighbors = [(y+yn,x+xn) for (yn,xn) in neighbors if 0 <= y+yn < height and 0 <= x+xn < width]
    for (yn, xn) in valid_neighbors:
        input[yn,xn] += 1 if input[yn,xn] >= 0 else 0

def step(input):
    height, width = input.shape

    input += 1

    flashes = 0
    changed = True
    synchronized = False
    while changed:
        changed = False
        for y in range(0, height):
            for x in range(0, width):
                if input[y,x] > 9:
                    input[y,x] = -1
                    flash(input, y, x)
                    flashes += 1
                    changed = True
    
    for y in range(0, height):
        for x in range(0, width):
            if input[y,x] == -1:
                input[y,x] = 0
    
    return flashes

input = np.genfromtxt('inputs.txt', dtype='i', delimiter=1)

i = 0
flashes = 0

while True:
    new_flashes = step(input)
    if new_flashes == input.size:
        print("Part 2: {} steps".format(i+1))
        if i >= 100:
            break
    
    flashes += new_flashes
    if i == 99:
        print("Part 1: {} flashes".format(flashes))
    
    i += 1
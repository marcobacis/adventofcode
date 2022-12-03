# Read inputs
with open('inputs.txt','r') as f:
    positions = [int(n) for n in f.readline().split(',')]

min_pos = min(positions)
max_pos = max(positions)

costs = [sum([abs(x - pos) for x in positions]) for pos in range(min_pos, max_pos + 1)]

print(min(costs))
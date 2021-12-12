# Constants
new_lifespan = 9
old_lifespan = 7

# Read inputs
with open('inputs.txt','r') as f:
    line = f.readline()
numbers = [int(n) for n in line.split(',')]

# Each position represents a timer value [0 to 8]
fishes = [len([1 for n in numbers if n == i]) for i in range(0, new_lifespan)]

# How many fishes after 256 days? A lot..
n_steps = 256

for i in range(n_steps):
    spawns = fishes[0]
    fishes = fishes[1:] + [spawns]
    fishes[old_lifespan - 1] += spawns

print(sum(fishes))

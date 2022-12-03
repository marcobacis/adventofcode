import io

with open('input.txt') as f:
    data = [(s.split()[0],int(s.split()[1])) for s in f.readlines()]

horizontal = 0
vertical = 0
aim = 0

for (cmd, val) in data:
    if cmd == "forward":
        horizontal +=val
        vertical += val * aim
    elif cmd == "down":
        aim += val
    elif cmd == "up":
        aim -= val


print(horizontal * vertical)
# Read inputs

with open('input.txt','r') as f:
    lines = f.readlines()

lines = [l.split("\n")[0].split(" -> ") for l in lines]
lines = [l[0].split(',') + l[1].split(',') for l in lines]

x1 = [int(l[0]) for l in lines]
x2 = [int(l[2]) for l in lines]
y1 = [int(l[1]) for l in lines]
y2 = [int(l[3]) for l in lines]
x_max = max(max(x1), max(x2)) + 1
y_max = max(max(y1), max(y2)) + 1

marks = [0] * x_max * y_max

def print_marks(marks, x_max, y_max):
    for y in range(0, y_max):
        print(marks[x_max * y : x_max * (y+1)])

def mark_line(x1, y1, x2, y2, marks, x_max):
    start_x = min(int(x1),int(x2))
    start_y = min(int(y1),int(y2))
    end_x = max(int(x1), int(x2))
    end_y = max(int(y1),int(y2))

    if start_x != end_x and start_y != end_y:
        return

    for y in range(start_x, end_x+1):
        for x in range(start_y, end_y+1):
            marks[y * x_max + x] += 1

for (x1p,y1p,x2p,y2p) in lines:
    mark_line(x1p,y1p,x2p,y2p, marks, x_max)

print(len([m for m in marks if m >= 2]))

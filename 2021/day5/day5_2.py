# Read inputs
def parse_line(line):
    coords = line.split(" -> ")
    coords = coords[0].split(',') + coords[1].split(',')
    return [int(n) for n in coords]

def read_lines(file_name):
    with open(file_name, 'r') as f:
        return [parse_line(l) for l in f.read().splitlines()]

# Function to mark lines, assuming horizontal, vertical or 45 degrees
def mark_line(line, marks, x_max):
    x1,y1,x2,y2 = line
    lx = x2 - x1
    ly = y2 - y1
    length = max(abs(lx),abs(ly))
    dx = abs(lx) / lx if lx != 0 else 0
    dy = abs(ly) / ly if ly != 0 else 0

    x = x1
    y = y1
    for i in range(0, length+1):
        marks[y * x_max + x] += 1
        x += dx
        y += dy


# Main
lines = read_lines('input.txt')
x_max = max([max(x1,x2) for (x1,_,x2,_) in lines]) + 1
y_max = max([max(y1,y2) for (_,y1,_,y2) in lines]) + 1
marks = [0] * x_max * y_max

for line in lines:
    mark_line(line, marks, x_max)

print(len([m for m in marks if m >= 2]))

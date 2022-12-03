import re

class Probe:
    def __init__(self, initial_speed=(0,0), target_area=[0,0,0,0]):
        self.y = 0
        self.x = 0
        self.speed = initial_speed
        self.xmin, self.xmax, self.ymin, self.ymax = target_area
    
    def step(self):
        self.x += self.speed[0]
        self.y += self.speed[1]
        xs = self.speed[0]
        new_x_speed = 0 if xs == 0 else (xs - 1 if xs > 0 else xs + 1)
        new_y_speed = self.speed[1] - 1
        self.speed = (new_x_speed, new_y_speed)

    def in_target_x(self):
        return self.xmin <= self.x <= self.xmax

    def in_target_y(self):
        return self.ymin <= self.y <= self.ymax
    
    def in_target(self):
        return self.in_target_x() and self.in_target_y()

    def overshoot(self):
        return self.x >= self.xmax or self.y <= self.ymin


# Read Input
target_area = [int(x) for x in re.findall(r'-?\d+', open('input.txt','r').read())]
xmin, xmax, ymin, ymax = target_area

# Part 1, bruteforce
max_y = 0
for y in range(-500, 500):
    probe = Probe((0, y), target_area)
    ys = set([max_y])
    while not probe.in_target_y() and not probe.overshoot():
        probe.step()
        ys.add(probe.y)
    if probe.in_target_y():
        max_y = max(ys)
print(f"Part 1: {max_y}")

# Part 2, bruteforce
n = 0
for y in range(-abs(ymin), abs(ymin)):
    for x in range(1, 1000):
        probe = Probe((x,y), target_area)
        while not probe.in_target() and not probe.overshoot():
            probe.step()
        if probe.in_target():
            n += 1
print(f"Part 2: {n}")
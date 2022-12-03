import numpy as np

def read_input(filename):
    with open(filename, 'r') as f:
        lines = f.read().splitlines()
        sep_idx = lines.index("")

        coords = [l.split(",") for l in lines[:sep_idx]]
        coords = [(int(y), int(x)) for (x,y) in coords]
        x_max = max(x for (y,x) in coords)
        y_max = max(y for (y,x) in coords)

        paper = np.zeros((y_max+1, x_max+1), dtype='i')

        for (y,x) in coords:
            paper[y,x] = 1

        folds = [l.replace("fold along ", "").split("=") for l in lines[sep_idx+1 : ]]
        folds = [(axis, int(coord)) for (axis, coord) in folds]

        return (paper, folds)

def fold(paper, f):
    axis, coord = f
    axis = 0 if axis == "y" else 1

    first, _, second = np.split(paper, [coord, coord+1], axis)

    return first + np.flip(second, axis=axis)

def normalize(paper):
    paper[paper > 0] = 1

paper, folds = read_input("inputs.txt")

first = True
for f in folds:
    paper = fold(paper, f)
    
    if first:
        print("First part: " + str(np.count_nonzero(paper)))
        first = False

print("Part 2:")
for i in range(paper.shape[0]):
    print("".join(["#" if n > 0 else " " for n in paper[i]]))
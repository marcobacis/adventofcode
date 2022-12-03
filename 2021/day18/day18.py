import math

class Number:
    def __init__(self, value, parent=None, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right
        self.parent = parent
    
    def is_leaf(self):
        return self.left is None and self.right is None
    
    def is_pair(self):
        return not self.is_leaf() and self.left.is_leaf() and self.right.is_leaf()

    def to_string(self):
        if self.is_leaf():
            return str(self.value)
        else:
            return "[" + self.left.to_string() + ", " + self.right.to_string() + "]"

def parse_snail_number(string):
    current_num = ""
    num = Number(0)
    for c in string:
        if c.isdigit():
            current_num += c
        elif c == '[':
            num.left = Number(0, parent=num)
            num.right = Number(0, parent=num)
            num = num.left
            current_num = ""
        elif c == ',':
            num.value = int(current_num) if current_num != "" else 0
            current_num = ""
            num = num.parent.right
        elif c == ']':
            num.value = int(current_num) if current_num != "" else 0
            current_num = ""
            num = num.parent
    return num

def add_snail_numbers(left, right):
    new_number = Number(0, None, left, right)
    left.parent = new_number
    right.parent = new_number
    return new_number

def split_snail_number(number):
    number.left = Number(math.floor(number.value / 2), number)
    number.right = Number(math.ceil(number.value / 2), number)

def explode_snail_number(number):
    is_left = True
    if number.parent.left == number:
        number.parent.left = None
        is_left = True
    else:
        number.parent.right = None
        is_left = False

    # Add left value to rightmost element of the left subtree
    prev = number
    current = number.parent
    while current is not None and (current.left is None or current.left == prev):
        prev = current
        current = current.parent
    if current is not None and current.left is not None:
        current = current.left
    while current is not None and not current.is_leaf():
        if current.right is not None:
            current = current.right
        elif current.left is not None:
            current = current.left
    if current is not None and current.is_leaf():
        current.value += number.left.value
    
    # Add left value to rightmost element of the left subtree
    prev = number
    current = number.parent
    while current is not None and (current.right is None or current.right == prev):
        prev = current
        current = current.parent
    if current is not None and current.right is not None:
        current = current.right
    while current is not None and not current.is_leaf():
        if current.left is not None:
            current = current.left
        elif current.right is not None:
            current = current.right
    if current is not None and current.is_leaf():
        current.value += number.right.value
    
    if is_left:
        number.parent.left = number
    else:
        number.parent.right = number
    number.value = 0
    number.left = None
    number.right = None


def reduce(number):
    i = 0
    while reduce_internal(number):
        #print(f"Pass {i}: {number.to_string()}")
        i += 1
        pass
    
def reduce_internal(number):
    return reduce_explodes(number) or reduce_splits(number)

def reduce_splits(number):
    if number is None:
        return False
    if number.is_leaf() and number.value >= 10:
        split_snail_number(number)
        return True
    return reduce_splits(number.left) or reduce_splits(number.right)

def reduce_explodes(number, level=1):
    if number is None:
        return False
    
    if number.is_pair() and level > 4:
        #print(f"Exploding {number.to_string()}")
        explode_snail_number(number)
        return True
    
    return reduce_explodes(number.left, level+1) or reduce_explodes(number.right, level+1)

def magnitude(number):
    if number is None:
        return 0
    if number.is_leaf():
        return number.value
    else:
        return 3 * magnitude(number.left) + 2 * magnitude(number.right)

lines = open('input.txt').read().splitlines()
numbers = [parse_snail_number(line) for line in lines]
sum = numbers[0]
for n in numbers[1:]:
    sum = add_snail_numbers(sum, n)
    reduce(sum)

print(f"Part 1: {magnitude(sum)}")

max_magnitude = 0
pair = (0,0)
for i in range(len(numbers)):
    for j in range(len(numbers)):
        if i != j:
            sum = add_snail_numbers(parse_snail_number(lines[i]), parse_snail_number(lines[j]))
            reduce(sum)
            mag = magnitude(sum)
            if mag > max_magnitude:
                max_magnitude = mag
                pair = (i,j)

print(f"Part 2: {max_magnitude}")
from itertools import islice
import io

def window(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result

data = []
with open("input.txt") as f:
    data = [int(l) for l in f.readlines()]

sums = [sum(win) for win in window(data, n=3)]

print(sums)

print(sum([ 1 if i > 0 and sums[i] > sums[i-1] else 0 for i in range(1,len(sums))]))

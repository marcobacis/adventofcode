
# 1 segment : none
# 2 segments: 1
# 3 segments: 7
# 4 segments: 4
# 5 segments: 2,3,5
# 6 segments: 0,6,9
# 7 segments: 8
#      aaaa  
#     b    c 
#     b    c 
#      dddd  
#     e    f 
#     e    f 
#      gggg  

# Second puzzle: decode and sum the outputs numbers

with open('inputs.txt','r') as f:
    lines = f.readlines()

answer = 0
for line in lines:
    inputs = [frozenset(word) for word in line.split(" | ")[0].split()]
    outputs = [frozenset(word) for word in line.split(" | ")[1].split()]

    _1, _7, _4, *unknown, _8 = sorted(inputs, key=len)
    _6 = next(x for x in unknown if len(_8 - x) == 1 and len(x & _1) == 1) ; unknown.remove(_6)
    _5 = next(x for x in unknown if len(_8 - x) == 2 and len(_6 - x) == 1); unknown.remove(_5)
    _2 = next(x for x in unknown if len(_8 - x) == 2 and len(x & _1) == 1); unknown.remove(_2)
    _3 = next(x for x in unknown if len(_8 - x) == 2 and len(x - _7) == 2); unknown.remove(_3)
    _9 = _8 - (_2 - _3); unknown.remove(_9)
    _0 = unknown[0]

    chipher = {v: str(i) for i,v in enumerate([_0,_1,_2,_3,_4,_5,_6,_7,_8,_9])}

    num = int(''.join([chipher[x] for x in outputs]))
    answer += num
    #print(num)

print(answer)
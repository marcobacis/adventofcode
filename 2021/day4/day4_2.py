
with open('input.txt') as f:
    draws = f.readline().split('\n')[0].split(',')
    lines = f.readlines()

lines = [l.split("\n")[0].split()  for l in lines if l != "\n"]
boards = [lines[i] +  lines[i+1] + lines[i+2] + lines[i+3] + lines[i+4] for i in range(0,len(lines), 5)]

def mark_board(num, board):
    for i in range(0,5):
        for j in range(0,5):
            if board[i*5 + j] == num:
                board[i*5 + j] = "x"

def board_wins(board):
    # Rows
    for i in range(0,5):
        if len([1 for j in range(0,5) if board[i*5+j] == "x"]) == 5:
            return True
    #Cols
    for j in range(0,5):
        if len([1 for i in range(0,5) if board[i*5+j] == "x"]) == 5:
            return True
    
    return False

def score(num, board):
    not_marked = [int(n) for n in board if n != "x"]
    return sum(not_marked) * int(num)

for n in draws:
    for b in boards:
        mark_board(n,b)

    not_winning = [b for b in boards if not board_wins(b)]

    if(len(not_winning) == 0):
        print(score(n, boards[0]))
        break
    
    boards = not_winning
A = X = 1 # rock
B = Y = 2 # paper
C = Z = 3 # scissors

X = 0 
Y = 3
Z = 6

# do_i_win[opponent][me]
# -1 : lose 0 : draw 1 : win
do_i_win = [
    [0, 1, -1],
    [-1, 0, 1],
    [1, -1, 0], 
]

def convert(letter, letterA, letterB, letterC, A, B, C):
    if letter == letterA:
        return A
    elif letter == letterB:
        return B
    elif letter == letterC:
        return C


score = 0

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()


# Strips the newline character
for line in Lines:
    opponent_letter = line.strip().split(" ")[0]
    result = line.strip().split(" ")[1]
    
    opponent = convert(opponent_letter, "A", "B", "C", A, B, C)
    me = convert(my_letter, "X", "Y", "Z", X, Y, Z)
    
    score += me
    
    if do_i_win[opponent - 1][me - 1] == -1:
        score += 0
    elif do_i_win[opponent - 1][me - 1] == 0:
        score += 3
    elif do_i_win[opponent - 1][me - 1] == 1:
        score += 6


print(score)

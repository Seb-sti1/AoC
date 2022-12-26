A = 1 # rock
B = 2 # paper
C = 3 # scissors

X = 0 # lose
Y = 3 # draw
Z = 6 # win

# do_i_win[opponent - 1][me - 1]
# -1 : lose 0 : draw 1 : win
do_i_win = [
    [0, 1, -1],
    [-1, 0, 1],
    [1, -1, 0], 
]

# should_play[opponent - 1][result/3]
should_play = [
    [C, A, B],
    [A, B, C],
    [B, C, A], 
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
    result_letter = line.strip().split(" ")[1]
    
    opponent = convert(opponent_letter, "A", "B", "C", A, B, C)
    result = convert(result_letter, "X", "Y", "Z", X, Y, Z)
    
    score += result
    score += should_play[opponent - 1][int(result/3)] 


print(score)

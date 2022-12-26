n, m = 1000, 1000

pos = [[False for i in range(m)] for j in range(n)]

rope = [[500, 500] for i in range(10)]

(x, y) = rope[9]
pos[x][y] = True


def show_state(rope):
    for i in range(n):
        for j in range(m):

            S = "."

            for k in range(len(rope) - 1, -1, -1):

                if i == rope[k][0] and j == rope[k][1]:
                    if k != 0:
                        S = str(k)
                    else:
                        S = "H"
            print(S, end="")
        print()
    print()


def move_rope(xH, yH, xT, yT):
    if xH - xT > 1:
        xT += 1

        if yH > yT:
            yT += 1
        elif yH < yT:
            yT -= 1
    elif xH - xT < -1:
        xT += -1

        if yH > yT:
            yT += 1
        elif yH < yT:
            yT -= 1

    if yH - yT > 1:
        yT += 1

        if xH > xT:
            xT += 1
        elif xH < xT:
            xT -= 1
    elif yH - yT < -1:
        yT += - 1

        if xH > xT:
            xT += 1
        elif xH < xT:
            xT -= 1

    return (xT, yT)


show_state(rope)

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

i = 0

# Strips the newline character
for line in Lines:
    move = line.strip().split(" ")

    print(f"== {move[0]} {move[1]} ==")

    for i in range(int(move[1])):
        x, y = rope[0]

        # move the head
        if move[0] == "U":
            if x - 1 < 0:
                print("out")
                break
            rope[0][0] = max(0, x - 1)
        elif move[0] == "D":
            if x + 1 > n - 1:
                print("out")
                break
            rope[0][0] = min(n - 1, x + 1)
        elif move[0] == "R":
            if y + 1 > m - 1:
                print("out")
                break
            rope[0][1] = min(m - 1, y + 1)
        elif move[0] == "L":
            if y - 1 < 0:
                print("out")
                break
            rope[0][1] = max(0, y - 1)

        for i in range(len(rope) - 1):
            # move knots i+1 relative to i
            (rope[i + 1][0], rope[i + 1][1]) = move_rope(rope[i][0], rope[i][1], rope[i + 1][0], rope[i + 1][1])

        # print(rope)
        (x, y) = rope[9]
        pos[x][y] = True

        # show_state(rope)

score = 0

for i in range(n):
    for j in range(m):

        if pos[i][j]:
            # print("#", end="")
            score += 1
        # else:
        # print(".", end="")
    # print()

print(score)

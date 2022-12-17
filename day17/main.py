import re
import time

pieces = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (0, 1), (1, 0), (1, 1)]
]


class Matrix:

    def __init__(self):
        self.pos = (0, 0)
        self.height = 4
        self.data = [[0 for j in range(7)] for i in range(4)]

        self.current_piece = -1
        self.highest = 0
        self.removed_lines = 0

    def refresh_highest_rock(self):
        self.highest = 0

        for i in range(self.highest, self.height):
            if sum(self.data[i]) == 0:
                self.highest = i
                break

    def draw(self):
        for i in range(self.height - 1, -1, -1):
            print("|", end="")
            for j in range(7):

                if self.data[i][j] == 0:
                    print(".", end="")
                elif self.data[i][j] == 1:
                    print("#", end="")
                elif self.data[i][j] == 2:
                    print("@", end="")
                else:
                    print("?", end="")

            print("|")
        print("+-------+")

    def add_height(self, to_add):
        if to_add <= 0:
            return

        to_add = max(to_add, 100)

        for i in range(to_add):
            self.data.append([0 for j in range(7)])

        self.height += to_add

        N = 100000
        if self.height > N:
            self.height -= (N - 500)
            self.removed_lines += (N - 500)

            self.pos = (self.pos[0], self.pos[1] - (N - 500))
            self.data = self.data[(N - 500):]

    def spawn(self):
        self.current_piece = (self.current_piece + 1) % len(pieces)
        piece = pieces[self.current_piece]
        piece_height = max([y for x, y in piece]) + 1

        self.pos = 2, self.highest + 3

        self.add_height(self.highest + 3 + piece_height - self.height)

    def move(self, x, y, piece):
        new_pos = self.pos[0] + x, self.pos[1] + y

        # check collisions
        for position in piece:
            if 0 > position[1] + new_pos[1] or position[0] + new_pos[0] < 0 or position[0] + new_pos[0] >= 7 or\
                    self.data[new_pos[1] + position[1]][new_pos[0] + position[0]] == 1:
                return False

        self.pos = new_pos
        return True

    def end_move(self, piece):
        for position in piece:
            self.highest = max(self.highest, self.pos[1] + position[1] + 1)
            self.data[self.pos[1] + position[1]][self.pos[0] + position[0]] = 1



def iterate_line(lines):
    gaz = []

    for line in lines:
        if line.strip() != "":
            for dir in line:
                gaz.append(-1 if dir == "<" else 1)

    matrix = Matrix()

    rocks = 0
    gaz_idx = 0

    N = 2022
    T = max(int(N/100), 1_000)

    while rocks < N:
        a = time.time_ns()

        matrix.spawn()
        piece = pieces[matrix.current_piece]
        rocks += 1

        stopped = False

        t = time.time_ns()
        while not stopped:
            # push by gaz
            matrix.move(gaz[gaz_idx], 0, piece)
            gaz_idx = (gaz_idx + 1) % len(gaz)

            # move down
            stopped = not matrix.move(0, -1, piece)
        t = time.time_ns() - t

        matrix.end_move(piece)

        a = time.time_ns() - a

        if rocks % T == 0:
            print(rocks/N*100)

            percent_move = int(t / a * 100)
            print(f"total {t} move {percent_move}")

    print(matrix.highest + matrix.removed_lines)


file = open('input', 'r')
iterate_line(file.readlines())

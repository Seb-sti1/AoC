import re


class Map:
    def __init__(self, column):
        self.column = column
        self.row = 0
        self.data = []
        self.marks = []

    def add_line(self, line):
        self.marks.append([-1 for i in range(self.column)])

        data_line = []

        for i in range(self.column):

            if len(line) <= i or line[i] == " ":
                data_line.append(2)
            elif line[i] == ".":
                data_line.append(0)
            elif line[i] == "#":
                data_line.append(1)
            else:
                data_line.append(-1)


        self.data.append(data_line)
        self.row += 1

    def get_starting_point(self):
        pos = (0, 0, 0)

        while self.data[pos[0]][pos[1]] != 0:
            pos = (pos[0], pos[1] + 1, pos[2])

        return pos

    def wrap_around(self, pos):
        # 12
        # 3
        #45
        #6

        i, j, o = pos

        if i < 0 and j < 100 and o == 3:  # 1 top
            return j + 2 * 50, 0, 0  # ok

        if i < 50 and j == 49 and o == 2:  # 1 left
            return 50 - i + 2 * 50 - 1, 0, 0  # ok

        if i < 0 and j >= 100 and o == 3:  # 2 top
            return self.row - 1, j - 2 * 50, o  # ok

        if i < 50 and j == self.column and o == 0:  # 2 right
            return 50 - i + 2*50 - 1, 99, 2  # ok

        if i == 50 and j >= 100 and o == 1:  # 2 bot
            return j - 50, 99, 2  # ok

        if 50 <= i < 100 and j == 49 and o == 2:  # 3 left
            return 100, i - 50, 1  # ok

        if 50 <= i < 100 and j == 100 and o == 0:  # 3 right
            return 49, i + 50, 3  # ok

        if i == 99 and j < 50 and o == 3:  # 4 top
            return j + 50, 50, 0  # ok

        if 100 <= i < 150 and j < 0 and o == 2:  # 4 left
            return 50 - i + 2*50 - 1, 50, 0  # ok

        if 100 <= i < 150 and j == 100 and o == 0:  # 5 right
            return 50 - i + 2*50 - 1, 149, 2  # ok

        if i == 150 and 50 <= j < 100 and o == 1:  # 5 bot
            return j + 2*50, 49, 2  # ok

        if 150 <= i and j < 0 and o == 2:  # 6 left
            return 0, i - 100, 1  # ok

        if i == 200 and j < 50 and o == 1:  # 6 bot
            return 0, j + 100, 1  # ok

        if i >= 150 and j == 50 and o == 0:  # 6 right
            return 149, i - 150 + 50, 3

        return None

    def get_valid_pos(self, pos, new_pos):

        if 0 <= new_pos[0] < self.row and 0 <= new_pos[1] < self.column:
            if self.data[new_pos[0]][new_pos[1]] == 0:  # the new_pos is on a '.' position
                return new_pos
            elif self.data[new_pos[0]][new_pos[1]] == 1:  # the new_pos is on a '#' position
                return pos
            else:  # the new_pos in on a ' ' position
                return self.get_valid_pos(pos, self.wrap_around(new_pos))

        return self.get_valid_pos(pos, self.wrap_around(new_pos))

    def mark_pos(self, pos):
        self.marks[pos[0]][pos[1]] = pos[2]

    def move(self, pos, instruction):
        new_pos = None

        for i in range(instruction[0]):
            if pos[2] == 0:
                new_pos = (pos[0], pos[1] + 1, pos[2])
            elif pos[2] == 1:
                new_pos = (pos[0] + 1, pos[1], pos[2])
            elif pos[2] == 2:
                new_pos = (pos[0], pos[1] - 1, pos[2])
            elif pos[2] == 3:
                new_pos = (pos[0] - 1, pos[1], pos[2])

            pos = self.get_valid_pos(pos, new_pos)
            self.mark_pos(pos)
        #self.print()

        if instruction[1] == "L":
            pos = (pos[0], pos[1], (pos[2] - 1) % 4)
        elif instruction[1] == "R":
            pos = (pos[0], pos[1], (pos[2] + 1) % 4)

        return pos

    def print(self, with_marks=True):
        print()

        for i, data_line in enumerate(self.data):
            for j, pixel in enumerate(data_line):

                if with_marks and self.marks[i][j] != -1:
                    if self.marks[i][j] == 0:
                        print(">", end="")
                    elif self.marks[i][j] == 1:
                        print("v", end="")
                    elif self.marks[i][j] == 2:
                        print("<", end="")
                    elif self.marks[i][j] == 3:
                        print("^", end="")
                else:
                    if pixel == 0:
                        print(".", end="")
                    elif pixel == 1:
                        print("#", end="")
                    elif pixel == 2:
                        print(" ", end="")
                    else:
                        print("?", end="")
            print()
        print()

    def __str__(self):
        return f"Map {self.row} x {self.column}"


def iterate_line(lines):
    map_lines = []
    reading_map = True

    path = []

    for i, line in enumerate(lines):
        if line.strip() != "":

            if reading_map:
                map_lines.append(line.replace("\n", ""))
            else:
                matchs = re.findall(r"(\d+)([RL])?", line.strip())

                for m in matchs:
                    path.append((int(m[0]), m[1]))

        else:
            reading_map = False

    column = max([len(map_line) for map_line in map_lines])

    map = Map(column)
    for line in map_lines:
        map.add_line(line)

    pos = map.get_starting_point()
    map.mark_pos(pos)

    pos = map.move(pos, (10, ''))

    for instruction in path:
        pos = map.move(pos, instruction)
        map.mark_pos(pos)

    map.print()

    print(1000 * (pos[0] + 1) + 4 * (pos[1] + 1) + pos[2])


file = open('input', 'r')
iterate_line(file.readlines())

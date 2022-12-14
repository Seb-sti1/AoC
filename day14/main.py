
def draw_scan(scan):
    for line in scan:
        for s in line:
            if s == 0:
                print(".", end="")
            elif s == 1:
                print("#", end="")
            else:
                print("o", end="")

        print()


def iterate_sand(scan, x, y):

    n, m = len(scan), len(scan[0])

    if y == n - 1:
        return False

    if scan[y + 1][x] == 0:
        scan[y][x] = 0
        scan[y + 1][x] = 2
        return iterate_sand(scan, x, y + 1)

    if x == 0 or x == m - 1:
        return False

    if scan[y + 1][x - 1] == 0:
        scan[y][x] = 0
        scan[y + 1][x - 1] = 2
        return iterate_sand(scan, x - 1, y + 1)

    if scan[y + 1][x + 1] == 0:
        scan[y][x] = 0
        scan[y + 1][x + 1] = 2
        return iterate_sand(scan, x + 1, y + 1)

    return scan, x, y


def process(scan):
    i = 0
    while True:
        scan[0][500] = 2

        result = iterate_sand(scan, 500, 0)
        if not result:
            break

        scan, x, y = result

        i += 1

    print(i)


def iterate_line(lines):

    scan = [[0 for x in range(507)] for y in range(178)]

    for line in lines:
        if line.strip() != "":
            line = line.strip().replace(" ", "")

            points = line.split("->")

            for i in range(len(points) - 1):
                A = points[i].split(",")
                B = points[i + 1].split(",")

                y, x = (int(A[0]), int(A[1]))
                m, l = (int(B[0]), int(B[1]))

                if abs(x - l) > 0:
                    for j in range(min(x, l), max(x, l) + 1):
                        scan[j][y] = 1

                if abs(y - m) > 0:
                    for k in range(min(y, m), max(y, m) + 1):
                        scan[x][k] = 1

    process(scan)

file = open('input', 'r')
iterate_line(file.readlines())

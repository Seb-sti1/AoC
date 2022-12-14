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
        raise Exception("Grid too small")

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
        if scan[0][500] != 0:
            break
        scan[0][500] = 2

        result = iterate_sand(scan, 500, 0)
        if not result:
            break

        scan, x, y = result

        i += 1

    draw_scan(scan)
    print(i)


def iterate_line(lines):
    row, column = 180, 700

    scan = [[0 for x in range(column)] for y in range(row)]

    max_y = 0

    for line in lines:
        if line.strip() != "":
            line = line.strip().replace(" ", "")

            points = line.split("->")

            for i in range(len(points) - 1):
                A = points[i].split(",")
                B = points[i + 1].split(",")

                x, y = (int(A[0]), int(A[1]))
                l, m = (int(B[0]), int(B[1]))

                max_y = max(y, m, max_y)

                if abs(x - l) > 0:
                    for j in range(min(x, l), max(x, l) + 1):
                        scan[y][j] = 1

                if abs(y - m) > 0:
                    for k in range(min(y, m), max(y, m) + 1):
                        scan[k][x] = 1

    scan[max_y + 2] = [1 for x in range(column)]

    process(scan)


file = open('input', 'r')
iterate_line(file.readlines())

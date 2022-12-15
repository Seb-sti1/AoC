import re


def draw_scan(scan):
    for line in scan:
        for s in line:
            if s == 0:
                print(".", end="")
            elif s == 1:
                print("S", end="")
            elif s == 2:
                print("B", end="")
            elif s == 3:
                print("#", end="")
            else:
                print("?", end="")

        print()


def find_known_position(y, k, l, n, m, sensors, beacons):
    line = [0 for x in range(n - k + 1)]

    for idx, S in enumerate(sensors):
        if S[1] == y:
            line[S[0] - k] = 1

        B = beacons[idx]

        if B[1] == y:
            line[B[0] - k] = 2

        dist = abs(B[0] - S[0]) + abs(B[1] - S[1])
        from_y = abs(y - S[1])

        for i in range(- dist + from_y, dist - from_y + 1):
            if k <= S[0] + i <= n:
                if line[S[0] + i - k] == 0:
                    line[S[0] + i - k] = 3
            else:
                raise Exception(f"Too small ! {S[0] + i - k} {n - S[0] - i}")
    #  draw_scan([line])

    score = 0
    for e in line:
        if e == 3:
            score += 1

    return score


def iterate_line(lines):
    sensors = []
    beacons = []

    for line in lines:
        if line.strip() != "":
            coords = re.findall("[xy]=(-?\\d+)", line.strip())

            sensors.append((int(coords[0]), int(coords[1])))

            x, y = int(coords[2]), int(coords[3])
            #  if (x, y) not in beacons:
            beacons.append((x, y))

    k, l = min([s[0] for s in sensors] + [b[0] for b in beacons]) - 700176, \
        min([s[1] for s in sensors] + [b[1] for b in beacons])
    n, m = max([s[0] for s in sensors] + [b[0] for b in beacons]), \
        max([s[1] for s in sensors] + [b[1] for b in beacons])

    score = find_known_position(2_000_000, k, l, n, m, sensors, beacons)

    print(score)


file = open('test_input', 'r')
iterate_line(file.readlines())

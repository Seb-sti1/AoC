S = 0


def end_cycle(cycle, x):
    global S

    cycles = [20, 60, 100, 140, 180, 220]

    if cycle in cycles:
        print(x)
        S += cycle * x


def iterate_line(lines):
    x = 1
    cycle = 0

    for line in lines:
        line = line.strip().split(" ")

        if line[0] == "addx":
            cycle += 1
            end_cycle(cycle, x)

            cycle += 1
            end_cycle(cycle, x)
            x += int(line[1])
        else:
            cycle += 1
            end_cycle(cycle, x)


file = open('input', 'r')
iterate_line(file.readlines())

print(S)

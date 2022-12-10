S = 0
CRT = ""


def end_cycle(cycle, x):
    global CRT

    if x <= cycle % 40 <= x + 2:
        CRT += "#"
    else:
        CRT += "."

    if cycle % 40 == 0:
        CRT += "\n"


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

print(CRT)

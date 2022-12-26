S = 0
CRT = ""


def end_cycle(cycle, x):
    global CRT

    cursor_pos = cycle

    while cursor_pos > 40:
        cursor_pos -= 40

    print(f"{cycle} {cursor_pos}")

    if x <= cursor_pos <= x + 2:
        CRT += "#"
    else:
        CRT += "."

    if cycle % 40 == 0:
        CRT += "\n"


def iterate_line(lines):
    x = 1
    cycle = 0

    for line in lines:
        if line.strip() != "":
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

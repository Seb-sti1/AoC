class Number:
    def __init__(self, n, order_of_mixing):
        self.n = n
        self.order_of_mixing = order_of_mixing

    def __str__(self):
        return str(self.n)


def mixing(numbers):
    len_numbers = len(numbers)
    moved = 0

    while moved < len_numbers:
        if moved % 1000 == 0:
            print(moved / len_numbers * 100)

        to_moved = -1

        for idx in range(len_numbers):
            if numbers[idx].order_of_mixing == moved:
                to_moved = idx
                break

        if to_moved == -1:
            break

        N = numbers.pop(to_moved)
        N.moved = True
        numbers.insert((to_moved + N.n) % len(numbers), N)

        moved += 1
        

def show_list(numbers):
    print(", ".join([str(N) for N in numbers]))


def iterate_line(lines):
    numbers = []

    for i, line in enumerate(lines):
        if line.strip() != "":
            n = int(line.strip())

            numbers.append(Number(n*811589153, i))  # n*811589153

    #show_list(numbers)

    for i in range(10):
        mixing(numbers)

    # Find the zero
    position_of_zero = -1
    for i in range(len(numbers)):
        if numbers[i].n == 0:
            position_of_zero = i
            break

    # Look for key position
    numbers_to_look = [1000, 2000, 3000]
    numbers_found = [-1, -1, -1]

    for i, number_to_look in enumerate(numbers_to_look):
        pos = position_of_zero + number_to_look
        pos = pos % len(numbers)
        numbers_found[i] = numbers[pos].n

    # print out the score
    print(numbers_found)
    print(sum(numbers_found))


file = open('input', 'r')
iterate_line(file.readlines())

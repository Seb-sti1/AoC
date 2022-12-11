class Monkey:
    idx = None
    inspected_items = 0
    items = []

    def __init__(self, idx):
        self.idx = idx
        self.false_monkey = None
        self.true_monkey = None
        self.test = None
        self.operation = None

    def set_starting_items(self, starting_items):
        self.items = starting_items

    def set_operation(self, operation):
        self.operation = operation

    def set_test(self, test):
        self.test = test

    def set_true_monkey(self, true_monkey):
        self.true_monkey = true_monkey

    def set_false_monkey(self, false_monkey):
        self.false_monkey = false_monkey


monkeys = []


def iterate_line(lines):
    global monkeys

    monkey = None

    for line in lines:
        if line.strip() != "":
            words = line.strip().split(" ")

            if words[0] == "Monkey":
                monkey = Monkey(len(monkeys))
            elif words[0] == "Starting":
                monkey.set_starting_items([int(words[i].replace(',', '')) for i in range(2, len(words))])
            elif words[0] == "Operation:":
                arg = words[5]
                op = words[4]

                operation = ((lambda x: x * x) if arg == 'old' else
                             (lambda c: lambda x: x * c)(int(arg)) if op == '*' else
                             (lambda c: lambda x: x + c)(int(arg)))

                monkey.set_operation(operation)
            elif words[0] == "Test:":
                monkey.set_test(int(words[3]))
            elif words[0] == "If" and words[1] == "true:":
                monkey.set_true_monkey(int(words[5]))
            elif words[0] == "If" and words[1] == "false:":
                monkey.set_false_monkey(int(words[5]))
                monkeys.append(monkey)


file = open('input', 'r')
iterate_line(file.readlines())

mod = 1
for monkey in monkeys:
    mod *= monkey.test

for i in range(10_000):  # round
    if i % 100 == 0:
        print(f"Round {i + 1}")

    for monkey in monkeys:
        for item in monkey.items:
            monkey.inspected_items += 1
            item = monkey.operation(item)
            item = int((item % mod))

            if item % monkey.test == 0:
                monkeys[monkey.true_monkey].items.append(item)
            else:
                monkeys[monkey.false_monkey].items.append(item)

        monkey.items = []
    j = 0
    for monkey in monkeys:
        print(f"Monkey {j} {' '.join([str(item) for item in monkey.items])}")
        j += 1


j = 0
for monkey in monkeys:
    print(f"Monkey {j} inspected items {monkey.inspected_items} times")
    j += 1

result = max([monkey.inspected_items for monkey in monkeys])
result *= max([monkey.inspected_items for monkey in monkeys if monkey.inspected_items != result])

print(result)

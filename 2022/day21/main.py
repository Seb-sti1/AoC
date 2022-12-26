class Monkey:
    def __init__(self, name, operation_s):
        self.name = name
        self.operation_s = operation_s

        self.value = -1
        self.necessary_monkeys = []

        words = self.operation_s.split(" ")
        if len(words) > 1:
            self.necessary_monkeys.append(words[0])
            self.necessary_monkeys.append(words[2])
        else:
            self.value = int(words[0])

    def __str__(self):
        if self.value == -1:
            return f"{self.name}: {self.operation_s}"
        return f"{self.name}: {self.value}"


def dict_and(dict):
    AND = True
    for key in dict:
        AND = AND and dict[key]

        if not AND:
            return False

    return AND


def all_necessary_monkeys(search_list, necessary_monkeys, ignore_monkeys):
    # create a dict
    found = {}
    for necessary_monkey in necessary_monkeys:
        if necessary_monkey in ignore_monkeys:
            found[necessary_monkey] = True
        else:
            found[necessary_monkey] = False

    # search for necessary monkeys
    for monkey in search_list:
        if monkey.name in found:
            found[monkey.name] = True

            if dict_and(found):
                return True

    return dict_and(found)


def sorted_fusion(sorted_list, to_add, ignore_monkeys):
    new_sorted_list = sorted_list + []
    new_to_add = []

    for monkey in to_add:

        if all_necessary_monkeys(new_sorted_list, monkey.necessary_monkeys, ignore_monkeys):
            new_sorted_list.append(monkey)
        else:
            new_to_add.append(monkey)

    if len(to_add) == len(new_to_add):
        return sorted_list, new_to_add

    return sorted_fusion(new_sorted_list, new_to_add, ignore_monkeys)


def eval_monkeys(sorted_list, default_values=None):
    if default_values is None:
        values = {}
    else:
        values = default_values.copy()

    for M in sorted_list:
        if M.name not in values:
            operation = M.operation_s

            for necessary_monkey in M.necessary_monkeys:
                if necessary_monkey not in values:
                    raise Exception("No values")

                operation = operation.replace(necessary_monkey, str(values[necessary_monkey]))

            M.value = int(eval(operation))

        values[M.name] = M.value

    return values


def iterate_line(lines):
    monkeys = []

    cant_eval_monkeys = []

    for i, line in enumerate(lines):
        if line.strip() != "":
            name, operation_s = line.strip().split(": ")

            if name == 'humn':
                continue
            if name == 'root':
                operation_s = operation_s.replace("+", "-")

            M = Monkey(name, operation_s)
            if len(M.necessary_monkeys) == 0:
                monkeys.append(M)
            else:
                if all_necessary_monkeys(monkeys, M.necessary_monkeys, []):
                    monkeys.append(M)
                else:
                    cant_eval_monkeys.append(M)

    monkeys_sorted, need_humn = sorted_fusion(monkeys, cant_eval_monkeys, [])
    values = eval_monkeys(monkeys_sorted)

    humn_monkey = Monkey('humn', str(-1))
    monkeys_sorted, empty = sorted_fusion([humn_monkey], need_humn, values.keys())

    if len(empty) > 0:
        raise Exception('Should be empty')

    monkeys_sorted.pop(0)

    S = 3_378_273_370_000
    N = 1_000_000_000_000_000

    min_root = 112609112355996
    min_humn = 0

    for humn in range(S, N, 1):

        values['humn'] = humn

        test_values = eval_monkeys(monkeys_sorted, values)

        if humn % 10_000 == 0:
            print(f"{(humn - S)/(N-S)*100} %")
            print(f"humn = {humn}, root = {test_values['root']}")
            print(f"Min : {min_root}, {min_humn}")

        if test_values['root'] < min_root:
            min_root = test_values['root']
            min_humn = humn

        if test_values['root'] == 0:
            print(f"humn = {humn}, root = {test_values['root']}")
            break


file = open('input', 'r')
iterate_line(file.readlines())

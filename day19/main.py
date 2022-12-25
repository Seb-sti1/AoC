import re
import time


class Blueprint:
    def __init__(self, id, ore_robot, clay_robot, obsidian_robot, geode_robot):
        self.id = id

        self.robot_recipes = []

        self.robot_recipes.append(ore_robot)
        self.robot_recipes.append(clay_robot)
        self.robot_recipes.append(obsidian_robot)
        self.robot_recipes.append(geode_robot)

        self.max_conso = None


class Materials:
    def __init__(self):
        self.ore = 0
        self.clay = 0
        self.obsidian = 0
        self.geode = 0

    def copy(self):
        n = Materials()
        n.ore = self.ore
        n.clay = self.clay
        n.obsidian = self.obsidian
        n.geode = self.geode

        return n

    def __str__(self):
        return f"Materials : ore {self.ore} clay {self.clay} obsidian {self.obsidian} geode {self.geode}"

    def __eq__(self, other):
        return self.ore == other.ore and self.clay == other.clay and \
            self.obsidian == other.obsidian and self.geode == other.geode

    def to_tuple(self):
        return self.ore, self.clay, self.obsidian, self.geode


class Robots:
    def __init__(self, ore, clay, obsidian, geode):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode

    def add(self, t, n):
        if t == 0:
            self.ore += n
        elif t == 1:
            self.clay += n
        elif t == 2:
            self.obsidian += n
        elif t == 3:
            self.geode += n

    def copy(self):
        return Robots(self.ore, self.clay, self.obsidian, self.geode)

    def __str__(self):
        return f"Robots : ore {self.ore} clay {self.clay} obsidian {self.obsidian} geode {self.geode}"

    def __eq__(self, other):
        return self.ore == other.ore and self.clay == other.clay and \
            self.obsidian == other.obsidian and self.geode == other.geode

    def to_tuple(self):
        return self.ore, self.clay, self.obsidian, self.geode


def sum_prod(a, b):
    result = 0

    for i in range(len(a)):
        result += a[i] * b[i]

    return result


class State:
    def __init__(self, robots, materials, blueprint, time_left):
        self.robots = robots
        self.materials = materials
        self.blueprint = blueprint
        self.time_left = time_left

    def next_states(self):
        acceptable_states = []

        max_conso = self.blueprint.max_conso

        new_materials = self.materials.copy()
        new_materials.ore += self.robots.ore
        new_materials.clay += self.robots.clay
        new_materials.obsidian += self.robots.obsidian
        new_materials.geode += self.robots.geode

        if (self.materials.ore <= max_conso.ore and self.robots.ore > 0) \
                or (self.materials.clay <= max_conso.clay and self.robots.clay > 0) \
                or (self.materials.obsidian <= max_conso.obsidian and self.robots.obsidian > 0) \
                or self.robots.geode > 0:
            acceptable_states.append(State(self.robots.copy(), new_materials, self.blueprint, self.time_left - 1))

        for i, recipe in enumerate(self.blueprint.robot_recipes):
            if i == 0 and self.robots.ore >= max_conso.ore:
                continue

            if i == 1 and self.robots.clay >= max_conso.clay:
                continue

            if i == 2 and self.robots.obsidian >= max_conso.obsidian:
                continue

            if recipe[0] <= self.materials.ore and recipe[1] <= self.materials.clay and \
                    recipe[2] <= self.materials.obsidian:
                new_materials_with_new_robot = new_materials.copy()
                new_materials_with_new_robot.ore -= recipe[0]
                new_materials_with_new_robot.clay -= recipe[1]
                new_materials_with_new_robot.obsidian -= recipe[2]

                new_robots = self.robots.copy()
                new_robots.add(i, 1)

                acceptable_states.append(State(new_robots, new_materials_with_new_robot,
                                               self.blueprint, self.time_left - 1))

        return acceptable_states[::-1]

    def theoretical_max(self):
        max_geodes = self.materials.geode

        j = 0
        for i in range(self.time_left, -1, -1):
            max_geodes += (self.robots.geode + j) * i
            j += 1

        return max_geodes

    def __str__(self):
        return f"{str(self.robots)} ; {str(self.materials)} ; {self.time_left}"

    def __eq__(self, other):
        if isinstance(other, tuple):
            return self.to_tuple() == other
        return self.robots == other.robots and self.materials == other.materials

    def to_tuple(self):
        return self.robots.to_tuple() + self.materials.to_tuple() + (self.time_left,)


def dfs(state, max_geodes):
    if state.time_left == 0:
        return max(state.materials.geode, max_geodes)

    for new_state in state.next_states():

        if max_geodes < 0 or new_state.theoretical_max() >= max_geodes:
            max_geodes = max(dfs(new_state, max_geodes), max_geodes)

    return max_geodes


def iterate_line(lines):
    blueprints = []

    for line in lines:
        if line.strip() != "":
            b = re.search(
                r"^Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.$",
                line.strip())

            blueprints.append(Blueprint(int(b[1]),
                                        (int(b[2]), 0, 0),
                                        (int(b[3]), 0, 0),
                                        (int(b[4]), int(b[5]), 0),
                                        (int(b[6]), 0, int(b[7]))
                                        ))

    score = 0
    for blueprint in blueprints:
        max_conso = Materials()

        for recipe in blueprint.robot_recipes:
            max_conso.ore = max(max_conso.ore, recipe[0])
            max_conso.clay = max(max_conso.clay, recipe[1])
            max_conso.obsidian = max(max_conso.obsidian, recipe[2])

        blueprint.max_conso = max_conso

        max_geodes = dfs(State(Robots(1, 0, 0, 0), Materials(), blueprint, 24), 0)
        print(f"Max geodes for blueprint {blueprint.id} is {max_geodes}")

        score += blueprint.id * max_geodes

    print(score)


file = open('input', 'r')
t = time.time()
iterate_line(file.readlines())
print(f"it took {time.time() - t}")

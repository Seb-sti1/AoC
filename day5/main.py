"""
                [B] [L]     [J]    
            [B] [Q] [R]     [D] [T]
            [G] [H] [H] [M] [N] [F]
        [J] [N] [D] [F] [J] [H] [B]
    [Q] [F] [W] [S] [V] [N] [F] [N]
[W] [N] [H] [M] [L] [B] [R] [T] [Q]
[L] [T] [C] [R] [R] [J] [W] [Z] [L]
[S] [J] [S] [T] [T] [M] [D] [B] [H]
 1   2   3   4   5   6   7   8   9 
"""

crates = [["S", "L", "W"],
["J", "T", "N" , "Q"],
["S", "C", "H", "F", "J"],
["T", "R", "M", "W", "N", "G", "B"],
["T", "R", "L", "S", "D", "H", "Q", "B"],
["M", "J", "B", "V", "F", "H", "R", "L"],
["D", "W", "R", "N", "J", "M"],
["B", "Z", "T", "F", "H", "N", "D", "J"],
["H", "L", "Q", "N", "B", "F", "T"]
]

# Using readlines()
file1 = open('input_clear', 'r')
Lines = file1.readlines()

# Strips the newline character
for line in Lines:
    sp = line.strip().split(" ")
    
    nb = int(sp[1])
    f = int(sp[3]) - 1
    t = int(sp[5]) - 1
    
    for i in range(nb):
        crates[t].append(crates[f][-nb + i])
        
    for i in range(nb):
        crates[f].pop(-1)
    	
print(crates)

print()

for stack in crates:
    print(stack[-1], end="")
    
print()


Elf = [0]

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

ElfIdx = 0
# Strips the newline character
for line in Lines:
    if line.strip() == "":
        ElfIdx += 1
        Elf.append(0)
    else:
        Elf[ElfIdx] += int(line.strip())
    

S = 0

for i in range(3):
    M = max(Elf)
    print(f"{i+1}) {M}")
    S += M
    
    Elf.pop(Elf.index(M))
    
    
print(S)

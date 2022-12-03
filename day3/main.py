score = 0

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

# Strips the newline character
for i in range(0, len(Lines), 3):
    elve1 = Lines[i].strip()
    elve2 = Lines[i+1].strip()
    elve3 = Lines[i+2].strip()
    
    inter = list(filter(lambda x: x in elve1, elve2))
    inter = list(filter(lambda x: x in inter, elve3))
    
    s = ord(inter[0])
    
    if s <= ord("Z"):
    	s += 27 - ord("A")
    else:
    	s += 1 - ord("a")

    score += s

print(score)

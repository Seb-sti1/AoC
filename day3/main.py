
score = 0

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()




# Strips the newline character
for line in Lines:
    letters = line.strip()
    
    first_comp = letters[0:int(len(letters)/2)]
    second_comp = letters[int(len(letters)/2):len(letters)]
    
    
    inter = list(filter(lambda x: x in first_comp, second_comp))
    
    s = ord(inter[0])
    
    if s <= ord("Z"):
    	s += 27 - ord("A")
    else:
    	s += 1 - ord("a")

    score += s
    
    print(inter, s)

print(score)

score = 0

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

# Strips the newline character
for line in Lines:
    elve1 = line.strip().split(",")[0]
    elve2 = line.strip().split(",")[1]
    
    elve1_range = range(int(elve1.split("-")[0]), int(elve1.split("-")[1]) + 1)
    elve2_range = range(int(elve2.split("-")[0]), int(elve2.split("-")[1]) + 1)
    
    
    inter = list(filter(lambda x: x in elve1_range, elve2_range))
    print(elve1_range, elve2_range, inter)
    
    
    if len(inter) > 0:
        print("ok")
        score += 1
    	

print(score)

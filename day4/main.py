score = 0

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

# Strips the newline character
for line in Lines:
    elve1 = line.strip().split(",")[0]
    elve2 = line.strip().split(",")[1]
    
    elve1_range = (int(elve1.split("-")[0]), int(elve1.split("-")[1]))
    elve2_range = (int(elve2.split("-")[0]), int(elve2.split("-")[1]))
    
    print(elve1_range, elve2_range)
    
    
    if elve1_range[0] <= elve2_range[0] and elve2_range[1] <= elve1_range[1]:
        print("ok")
        score += 1
    	
    if elve2_range[0] <= elve1_range[0] and elve1_range[1] <= elve2_range[1] and (elve2_range[0] != elve1_range[0] or elve1_range[1] != elve2_range[1]):
        print("ok 2")
        score += 1
    	

print(score)

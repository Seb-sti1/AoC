# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

def all_diff(cars):
    
    for i in range(len(cars)):
        for j in range(i +1, len(cars)):
            if cars[i] == cars[j]:
                return False
                
    return True


# Strips the newline character
for line in Lines:
    data = line.strip()
    L = 4
    
    for i in range(len(data) - L):
    	
        if all_diff(data[i:i+L]):
            print(i+L)
            break
    
    
    	


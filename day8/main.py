trees = []

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

i = 0

# Strips the newline character
for line in Lines:
    tree_raw = line.strip()
    
    trees.append([])
    
    for tree in tree_raw:
    	trees[i].append(int(tree))
    	
    i += 1

print(trees)

S = 2*len(trees) + 2*(len(trees[0]) - 2)


visible_map = [[0 for e in tree] for tree in trees]

for i in range(1, len(trees) - 1):
    for j in range(1, len(trees[0]) - 1):
        
        visible = True
        for a in range(i):
            if trees[a][j] >= trees[i][j]:
                visible = False
                break
                
        if visible:
            S += 1
            visible_map[i][j] = 1
        else:
            visible = True
            for a in range(i + 1, len(trees)):
                if trees[a][j] >= trees[i][j]:
                    visible = False
                    break
            
            if visible:
                S += 1
                visible_map[i][j] = 1
            else:
                visible = True
                for a in range(j):
                    if trees[i][a] >= trees[i][j]:
                        visible = False
                        break
                        
                if visible:
                    S += 1
                    visible_map[i][j] = 1
                else:
                    visible = True
                    for a in range(j + 1, len(trees[0])):
                        if trees[i][a] >= trees[i][j]:
                            visible = False
                            break
                            
                    if visible:
                        S += 1
                        visible_map[i][j] = 1

for raw in visible_map:
    for ele in raw: 
        print(ele, end="")
    print()
           
print(S)   
                
                
                
                
                
                
                
                
                

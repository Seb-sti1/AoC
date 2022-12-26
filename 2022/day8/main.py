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


visible_map = [[0 for e in tree] for tree in trees]

highest = 0

for i in range(1, len(trees) - 1):
    for j in range(1, len(trees[0]) - 1):
        visibility = 1

        for a in range(i-1, -1, -1):
            if trees[a][j] >= trees[i][j]:
                break
        visibility *= abs(a - i + 1) + 1
                
        for a in range(i + 1, len(trees)):
            if trees[a][j] >= trees[i][j]:
                break
        visibility *= abs(a - i - 1) + 1

        for a in range(j - 1, -1, -1):
            if trees[i][a] >= trees[i][j]:
                break
        visibility *= abs(a - j + 1) + 1

                        
        for a in range(j + 1, len(trees[0])):
            if trees[i][a] >= trees[i][j]:
                break
        visibility *= abs(a - j - 1) + 1
                          
        visible_map[i][j] = visibility

        highest = max(highest, visibility)
                
        

for raw in visible_map:
    for ele in raw: 
        print(ele, end=",")
    print()
           
print(highest)
                
                
                
                
                
                
                
                
                

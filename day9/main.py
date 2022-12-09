n, m = 1000,1000

pos = [[False for i in range(m)] for j in range(n)]

H_T = [500, 500, 500, 500]

(x, y) = H_T[2], H_T[3] 
pos[x][y] = True

def show_state(H_T):

    for i in range(n):
        for j in range(m):
            
            S = "."
            
            if (i, j) == (H_T[2], H_T[3]):
                S = "T"
            
            if (i, j) == (H_T[0], H_T[1]):
                S = "H"
            
            print(S, end="")
        print()
        
    print()

#show_state(H_T)

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

i = 0

# Strips the newline character
for line in Lines:
    move = line.strip().split(" ")
    
    print(f"== {move[0]} {move[1]} ==")

    for i in range(int(move[1])):
        (x, y) = H_T[0], H_T[1]
    
        if move[0] == "U":
            if H_T[0] - 1 < 0:
                print("out")
                break
            H_T[0] = max(0, H_T[0] - 1)
        elif move[0] == "D":
            if H_T[0] + 1 > n-1:
                print("out")
                break
            H_T[0] = min(n-1, H_T[0] + 1)
        elif move[0] == "R":
            if H_T[1] + 1 > m-1:
                print("out")
                break
            H_T[1] = min(m-1, H_T[1] + 1)
        elif move[0] == "L":
            if H_T[1] - 1 < 0:
                print("out")
                break
            H_T[1] = max(0, H_T[1] - 1)
    
        (xH, yH, xT, yT) = H_T
        
        if xH - xT > 1:
            H_T[2] += 1
            
            if yH > yT:
                H_T[3] += 1
            elif yH < yT:
                H_T[3] -= 1
        elif xH - xT < -1:
            H_T[2] += -1
            
            if yH > yT:
                H_T[3] += 1
            elif yH < yT:
                H_T[3] -= 1
       
        if yH - yT > 1:
            H_T[3] += 1
            
            if xH > xT:
                H_T[2] += 1
            elif xH < xT:
                H_T[2] -= 1
        elif yH - yT < -1:
            H_T[3] += - 1
            
            if xH > xT:
                H_T[2] += 1
            elif xH < xT:
                H_T[2] -= 1


        (x, y) = H_T[2], H_T[3] 
        pos[x][y] = True
        
        #show_state(H_T)
        
        
score = 0

for i in range(n):
    for j in range(m):
         
        if pos[i][j]:
            #print("#", end="")
            score += 1
        #else:
            #print(".", end="")
    #print()

print(score)


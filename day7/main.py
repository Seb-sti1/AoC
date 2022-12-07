files = {}

dirs = []
seen_files = []

# Using readlines()
file1 = open('input', 'r')
Lines = file1.readlines()

current_dir = ""

def parent_dirs(path):
    dirs = path.split("/")
    parents = ["/".join(dirs[0:i]) for i in range(2, len(dirs))]
    return [child + "/" for child in parents]

i = 0

# Strips the newline character
for line in Lines:
    words = line.strip().split(" ")

    if words[0] == "$":

        if words[1] == "cd":
            if words[2] == "..":
                if current_dir != "/":
                    last_dir = current_dir.split("/")[-2]
                    current_dir = current_dir[:-len(last_dir)-1]
            elif words[2] == "/":
                current_dir = "/"
            else:
                current_dir += words[2] + "/"
            
            if not current_dir in dirs:
                dirs.append(current_dir)
                    
    elif words[0] == "dir":
        pass
    elif len(words) >= 2:
        if not current_dir + words[1] in seen_files:
            files[current_dir + words[1]] = int(words[0])


for path in files:
    print(f"{path} {files[path]}")
    
      
dirs = set(dirs)

S = 0

for d in dirs:
    dS = 0
    
    for path in files:
        if path.startswith(d):
            dS += files[path]
    
    #print(f"{d} {dS}")
    
    if dS < 100000:
        S += dS

print(S)


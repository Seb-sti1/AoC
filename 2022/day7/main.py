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

total_used = 0

for path in files:
    total_used += files[path]
    print(f"{path} {files[path]}")
    
total_unused = 70000000 - total_used
dirs = set(dirs)

candidate = -1

for d in dirs:
    dS = 0
    
    for path in files:
        if path.startswith(d):
            dS += files[path]
    
    if dS > 30000000 - total_unused and (candidate == -1 or dS < candidate):
        candidate = dS

print(candidate)



data = ""

with open("input.txt", "r") as f:
    data = f.read()


lines = data.split("\n")

elves = []

elf = []


for l in lines:
    if l.strip() == "":
        elves.append(elf)
        elf = []
    else:
        elf.append(l)

sums = []

for e in elves:
    sum = 0
    for n in e:
        val = int(n)
        sum += val
    sums.append(sum)


sums.sort(reverse=True)

print(sums[0] + sums[1] + sums[2])

import math

def fuel(mass):
    return (math.floor(mass/3)-2)

sum = 0
with open('1a.txt',"r") as f:
    for line in f.readlines():
        sum += fuel(int(line))

print(fuel(12))
print(fuel(14))
print(fuel(1969))

print(sum)

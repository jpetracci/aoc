import math

def fuel(mass):
    return (math.floor(mass/3)-2)


def fuelWithFuel(mass):
    _fuel = fuel(mass)
    if _fuel <= 0:
        return 0
    return _fuel + fuelWithFuel(_fuel)


with open('1b.txt', "r") as f:
    masses = [int(l) for l in f.readlines()]


print(sum(map(fuel,masses)))
print(sum(map(fuelWithFuel,masses)))


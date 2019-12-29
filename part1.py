
#global vars
floor = 0


def move(dir):
    global floor
    if dir == '(':
        floor += 1
    elif dir == ')':
        floor -= 1


def process(moves):
    global floor
    floor = 0
    [move(x) for x in moves]


def first_basement(moves):
    global floor
    floor = 0
    for idx, x in enumerate(moves):
        move(x)
        if floor < 0:
            return idx+1

# input data
with open('input.txt', mode='r') as f:
    data = f.readline()
    # part 1
    process(data)
    print(floor)

    # part 2
    print(first_basement(data))

# part 1 tests
process('(())')
assert(floor == 0)

process('()()')
assert(floor == 0)

process('(((')
assert(floor == 3)

process('(()(()(')
assert(floor == 3)

process('))(((((')
assert(floor == 3)

process('())')
assert(floor == -1)

process('))(')
assert(floor == -1)

process(')))')
assert(floor == -3)

process(')())())')
assert(floor == -3)

# part 2 tests
assert(first_basement(')') == 1)
assert(first_basement('()())') == 5)

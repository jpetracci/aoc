
#globals
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


# run with input data
with open('2015/day1/input.txt', mode='r') as f:
    data = f.readline()
    # part 1
    process(data)
    print(floor)

    # part 2
    print(first_basement(data))


'''
tests
'''


def run_test_p1(test_data):
    global floor
    for route, end_floor in test_data:
        process(route)
        assert(floor == end_floor)


def run_test_p2(test_data):
    global floor
    for route, basement_index in test_data:
        assert(first_basement(route) == basement_index)


# part 1 tests
test_data_p1 = [['(())', 0],
                ['()()', 0],
                ['(((', 3],
                ['(()(()(', 3],
                ['))(((((', 3],
                ['())', -1],
                ['))(', -1],
                [')))', -3],
                [')())())', -3]]

run_test_p1(test_data_p1)

# part 2 tests
test_data_p2 = [[')', 1],
                ['()())', 5]]

run_test_p2(test_data_p2)


def calc_wrapping_paper(demension):
    l, w, h = demension.split('x')

    l = int(l)
    w = int(w)
    h = int(h)

    smallest_area = min(l*w, l*h)
    smallest_area = min(smallest_area, w*h)

    return (2*l*w)+(2*w*h)+(2*h*l) + smallest_area


def calc_ribbon(demension):
    dem = demension.split('x')
    dem = list(map(int, dem))
    dem.sort()

    volume = dem[0]*dem[1]*dem[2]
    total = dem[0]+dem[0]+dem[1]+dem[1] + volume
    
    return total


with open("2015/day2/input.txt", 'r') as f:
    sum = 0
    ribbon = 0
    for dem in f.readlines():
        # part 1
        sum += calc_wrapping_paper(dem)

        # part 2
        ribbon += calc_ribbon(dem)

    print(f'total wrapping paper = {sum}')
    print(f'total ribbon = {ribbon}')

# part 1 tests
assert(calc_wrapping_paper('2x3x4') == 58)
assert(calc_wrapping_paper('1x1x10') == 43)

# part 2 tests
assert(calc_ribbon('2x3x4') == 34)
assert(calc_ribbon('1x1x10') == 14)

def performOps(op, i):
    try:
        if op[i] == 1:
            # add
            op[op[i+3]] = op[op[i+1]]+op[op[i+2]]

        elif op[i] == 2:
            # mult
            op[op[i+3]] = op[op[i+1]]*op[op[i+2]]

        elif op[i] == 99:
            # exit
            return op

        return performOps(op, i+4)

    except IndexError:
        return op


def findInputs(ops, num):
    pos1 = 0
    pos2 = 0
    temp = ops
    
    for p1 in range(0,100):
        for p2 in range(0,100):
            temp = ops.copy()
            temp[1] = p1
            temp[2] = p2
            performOps(temp,0)
            if temp[0] == num:
                pos1 = p1
                pos2 = p2

    return [pos1, pos2]


with open('2a.txt', 'r') as inputFile:
    raw = inputFile.readline()

print(raw)

opCodes = [int(o) for o in raw.split(',')]

res = findInputs(opCodes, 19690720)
print(100*res[0]+res[1])

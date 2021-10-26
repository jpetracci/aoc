
def performOps(op, i):
    if op[i] == 1:
        # add
        op[op[i+3]] = op[op[i+1]]+op[op[i+2]]

    elif op[i] == 2:
        # mult
        op[op[i+3]] = op[op[i+1]]*op[op[i+2]]
    
    elif op[i] == 99:
        #exit
        return op

    return performOps(op,i+4)


with open('2a.txt', 'r') as inputFile:
    raw = inputFile.readline()

print(raw)

opCodes = [int(o) for o in raw.split(',')]


test = performOps([1,0,0,0,99], 0)
print(test)
test = performOps([2,3,0,3,99], 0)
print(test)
test = performOps([2,4,4,5,99,0], 0)
print(test)
test = performOps([1,1,1,4,99,5,6,0,99], 0)
print(test)

opCodes[1] = 12
opCodes[2] = 2
final = performOps(opCodes, 0)
print(final)

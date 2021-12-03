from prelude import read

def average(matrix, i):
    return sum(elt[i] == '1' for elt in matrix) >= len(matrix) / 2

if __name__ == '__main__':
    data = read()
    n = len(data[0])

    # 1
    commons = [average(data, i) for i in range(n)]
    commons = eval('0b' + ''.join(str(int(i)) for i in commons))

    uncommons = [not average(data, i) for i in range(n)]
    uncommons = eval('0b' + ''.join(str(int(i)) for i in uncommons))

    print(commons * uncommons)

    # 2
    candidates = data[:]
    for i in range(n):
        if len(candidates) == 1:
            break
        test = str(int(average(candidates, i)))
        candidates = [elt for elt in candidates if elt[i] == test]
    generator = eval('0b' + candidates[0])

    candidates = data[:]
    for i in range(n):
        if len(candidates) == 1:
            break
        test = str(int(not average(candidates, i)))
        candidates = [elt for elt in candidates if elt[i] == test]
    scrubber = eval('0b' + candidates[0])

    print(generator * scrubber)

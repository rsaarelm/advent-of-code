from prelude import read

def update(pos, cmd):
    if cmd[0] == 'forward':
        pos[0] += int(cmd[1])
    elif cmd[0] == 'down':
        pos[1] += int(cmd[1])
    elif cmd[0] == 'up':
        pos[1] -= int(cmd[1])
    return pos

def update2(pos, cmd):
    x = int(cmd[1])
    if cmd[0] == 'forward':
        pos[0] += x
        pos[1] += pos[2] * x
    elif cmd[0] == 'down':
        pos[2] += x
    elif cmd[0] == 'up':
        pos[2] -= x
    return pos

if __name__ == '__main__':
    data = read(lambda x: x.split(' '))
    pos = [0, 0]
    for elt in data:
        pos = update(pos, elt)
    print(pos[0] * pos[1])

    pos = [0, 0, 0]
    for elt in data:
        pos = update2(pos, elt)
    print(pos[0] * pos[1])

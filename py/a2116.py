from math import prod
from dataclasses import dataclass
from prelude import read


@dataclass
class Packet:
    version: int
    id: int
    payload = None

    def version_sum(self):
        ret = self.version
        try:
            for packet in self.payload:
                ret += packet.version_sum()
        except TypeError:
            pass
        return ret

    def eval(self):
        if self.id == LITERAL:
            return self.payload

        elif self.id == SUM:
            return sum(c.eval() for c in self.payload)
        elif self.id == PRODUCT:
            return prod(c.eval() for c in self.payload)
        elif self.id == MINIMUM:
            return min(c.eval() for c in self.payload)
        elif self.id == MAXIMUM:
            return max(c.eval() for c in self.payload)
        elif self.id == GREATER:
            a, b = self.payload
            return int(a.eval() > b.eval())
        elif self.id == LESS:
            a, b = self.payload
            return int(a.eval() < b.eval())
        elif self.id == EQUAL:
            a, b = self.payload
            return int(a.eval() == b.eval())
        else:
            assert False

    def __str__(self):
        if self.id == LITERAL:
            return str(self.payload)

        elif self.id == SUM:
            return "(%s)" % (" + ".join(str(c) for c in self.payload))
        elif self.id == PRODUCT:
            return "(%s)" % (" * ".join(str(c) for c in self.payload))
        elif self.id == MINIMUM:
            return "min([%s])" % (", ".join(str(c) for c in self.payload))
        elif self.id == MAXIMUM:
            return "max([%s])" % (", ".join(str(c) for c in self.payload))
        elif self.id == GREATER:
            a, b = self.payload
            return "(%s > %s)" % (str(a), str(b))
        elif self.id == LESS:
            a, b = self.payload
            return "(%s < %s)" % (str(a), str(b))
        elif self.id == EQUAL:
            a, b = self.payload
            return "(%s == %s)" % (str(a), str(b))
        else:
            assert False


TOTAL_LENGTH = 0
NUM_PACKETS = 1

SUM = 0
PRODUCT = 1
MINIMUM = 2
MAXIMUM = 3
LITERAL = 4
GREATER = 5
LESS = 6
EQUAL = 7


class Bits:
    def __init__(self, hexes):
        def bits(hexes):
            for c in hexes.strip():
                for b in format(int(c, 16), "#06b")[2:]:
                    yield int(b)

        self.bits = bits(hexes)
        self.bitpos = 0

    def bit(self):
        self.bitpos += 1
        return next(self.bits)

    def nbit_num(self, n_bits):
        a = 0
        for i in range(n_bits - 1, -1, -1):
            a += self.bit() << i
        return a

    def number(self):
        ret = 0
        chunk = 0b10000
        while chunk & 0b10000:
            chunk = self.nbit_num(5)
            ret = (ret << 4) + (chunk & 0b1111)
        return ret

    def packet(self):
        version = self.nbit_num(3)
        id = self.nbit_num(3)
        ret = Packet(version=version, id=id)

        if id == LITERAL:
            num = self.number()
            ret.payload = num
            return ret

        length_type = self.bit()
        if length_type == TOTAL_LENGTH:
            total_length = self.nbit_num(15)
            endpos = self.bitpos + total_length

            ret.payload = []
            while self.bitpos < endpos:
                ret.payload.append(self.packet())
            assert self.bitpos == endpos

            return ret
        else:
            num_packets = self.nbit_num(11)
            ret.payload = [self.packet() for _ in range(num_packets)]
            return ret


def eval(data):
    packet = Bits(data).packet()
    return packet.eval()


def test(data, result):
    packet = Bits(data).packet()
    assert packet.eval() == result


test("C200B40A82", 3)
test("04005AC33890", 54)
test("880086C3E88112", 7)
test("CE00C43D881120", 9)
test("D8005AC2A8F0", 1)
test("F600BC2D8F", 0)
test("9C005AC2F8F0", 0)
test("9C0141080250320F1802104A08", 1)

if __name__ == "__main__":
    data = read()[0]

    packet = Bits(data).packet()

    print(packet.version_sum())
    print(packet.eval())

import re
import sys
from typing import List

# Sequence and board in eval
# Return 0 while board does not win
# When it wins, figure out the last sequence item that marked it for score
# Sum unmarked numbers (use set?)


def is_bingo(moves, board):
    # Extra check for part 2, last move must make this bingo.
    if moves[-1] not in [x for row in board for x in row]:
        return False

    for i in range(5):
        if len([x for x in board[i] if x in moves]) == 5:
            return True
        if len([board[j][i] for j in range(5) if board[j][i] in moves]) == 5:
            return True
    return False


def score(moves, board):
    if not is_bingo(moves, board):
        return 0
    # XXX: Assume we score promptly on the finishing move, this will break
    # otherwise.
    win_number = moves[-1]
    return win_number * sum(
        [x for row in board for x in row if x not in moves]
    )


if __name__ == "__main__":
    data = sys.stdin.read().rstrip()
    chunks = data.split("\n\n")
    input = [int(x) for x in chunks[0].split(",")]
    boards = [
        [
            [int(x) for x in re.split(r"\s+", y.rstrip()) if x]
            for y in b.split("\n")
        ]
        for b in chunks[1:]
    ]

    # 1
    for i in range(1, len(input)):
        moves: List[int] = input[:i]
        for board in boards:
            if is_bingo(moves, board):
                print(score(moves, board))
                break
        else:
            # Multi-for-loop break hack
            continue
        break

    # 2
    for i in range(1, len(input)):
        moves = input[:i]
        j = 0
        while j < len(boards):
            board_score = score(moves, boards[j])
            if board_score:
                if len(boards) == 1:
                    print(board_score)
                    break
                else:
                    del boards[j]
            else:
                j += 1
        else:
            # Multi-for-loop break hack
            continue
        break

import numpy as np


data = """8548335644
6576521782
1223677762
1284713113
6125654778
6435726842
5664175556
1445736556
2248473568
6451473526
"""

data = np.array([list(map(int, line)) for line in data.splitlines()])

total = 0
for i in range(1000):
    data += 1
    while (update := np.argwhere(data > 9)).size:
        total += len(update)
        for row, col in update:
            data[row, col] = 0

            target = data[max(row-1, 0):row+2, max(col-1, 0):col+2]
            target[target != 0] += 1

    if i == 99:
        print(total)
    if np.all(data == data[0, 0]):
        print(i+1)
        break

import random

n = 200000
print(n)
for i in range(3):
    for j in range(n):
        print('.', end="")
    print()

q = 200000
print(q)
for i in range(q):
    r = random.randint(1, 3)
    c = random.randint(1, n)
    print(r, c)
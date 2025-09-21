import random

t = 1
# k = random.randrange(1, 1000000000)
k = 1
n = random.randrange(1, 500000)
s = random.randrange(10)


print(t)
for tt in range(t):
    s = []
    start = True
    for i in range(n):
        if start:
            nxt = 0
            while nxt == 0:
                nxt = random.randrange(10)
            s.append(nxt)
            start = False
        else:
            nxt = random.randrange(10)
            s.append(nxt)
    print(k)
    # print(s)
    for c in s:
        print(c, end = "")
    print()
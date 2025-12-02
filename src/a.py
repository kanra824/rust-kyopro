import random

print(1)
n = 1000000 // 50
print(n)

s = []
for i in range(n):
    len = random.randint(1, 100)
    nows = []
    for j in range(len):
        c = ord('a') + random.randint(0, 25)
        print(chr(c), end="")
    print()

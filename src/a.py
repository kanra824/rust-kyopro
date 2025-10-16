from random import randint

n = 60
m = 1000000000
print(n, m)
for i in range(n):
    print(randint(0, m), end=" ")
print()
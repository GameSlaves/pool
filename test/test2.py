import game_lib_pool as pool
import random

a = pool.Pool()

keys = [None]

for x in range(1,1232190):
    keys.append(a.push(x))

for x in range(0,321):
    rand_i = random.randrange(1, 1232190)
    while keys[rand_i] == None:
        rand_i = random.randrange(1, 1232190)

    a.remove(keys[rand_i])
    keys[rand_i] = None

for x in range(1,1232190):
    if keys[x] == None:
        continue
    assert a.get(keys[x]) == x

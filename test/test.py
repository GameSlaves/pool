import game_lib_pool

a = game_lib_pool.Pool()
b = a.push(1)
a.remove(b)
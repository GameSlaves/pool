import game_lib_pool

a = game_lib_pool.Pool()

b1 = a.push(1)
b2 = a.push(2)
b3 = a.push(3)
b4 = a.push(4)
b5 = a.push(5)

a.remove(b2)
a.remove(b1)


print(a.get(b3))
print(a.get(b4))
print(a.get(b5))
print('-------should be 3,4,5---------')

b1 = a.push(1)
b2 = a.push(2)

print(a.get(b1))
print(a.get(b2))
print(a.get(b3))
print(a.get(b4))
print(a.get(b5))
print('-------should be 1,2,3,4,5 (orderless)---------')

for x in a:
    print(x)
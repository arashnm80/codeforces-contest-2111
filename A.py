# https://codeforces.com/contest/2111/problem/A

t = int(input())

def func(nums, x):
    nums = sorted(nums)
    nums[0] = min(2*nums[1] + 1, x)
    return nums


for i in range(t):
    x = int(input())
    nums = [0, 0, 0]
    
    counter = 0

    while nums != [x, x, x]:
        nums = func(nums, x)
        counter += 1
    
    print(counter)

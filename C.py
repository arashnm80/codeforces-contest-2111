# OOOOOOOXXXXXXXXXXXXXXXXXOOOOOOOOO
#        ^               ^
#        left max        right min
 
t = int(input())
 
for i in range(t):
    n = int(input())
    nums = list(map(int, input().split()))
 
    min_cost = 5 * 10**5 * n
 
    for j in range(n):
        # if it's the first number
        if j == 0:
            l_max = r_min = j
        # if it's not the first number
        else:
            # is equal with previous one
            if nums[j] == nums[j-1]:
                r_min = j # shift r_min one step to right
            else:
                l_max = r_min = j # shift both to right
 
        found_cost = (n - (r_min - l_max + 1)) * nums[r_min]
        min_cost = min(min_cost, found_cost)
    
    print(min_cost)

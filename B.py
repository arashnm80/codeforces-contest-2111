fibo = [0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89] # zero is extra in beginning

def find_test_case_answer(n, m):
    # 2 biggest cubes based on n
    z = fibo[n]
    y = fibo[n-1]
    optimized_structure = [z, z, z+y]

    answer = []
    for j in range(m):
        # get 3 dimensions of box
        box_dimensions = list(map(int, input().split()))
        box_dimensions = sorted(box_dimensions)

        can_fit = True
        for i in range(3):
            if box_dimensions[i] < optimized_structure[i]:
                can_fit = False
                break
                
        if can_fit:
            answer.append(1)
        else:
            answer.append(0)
    
    return "".join(map(str, answer))

# number of test cases
t = int(input())

for i in range(t):
    n, m = map(int, input().split())
    print(find_test_case_answer(n,m))

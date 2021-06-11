import math

def permute(n, A):
    if n == 1:
        print(A)
        return

    permute(n-1, A)
    for i in range(n-1):
        if n % 2 == 0:
            temp = A[n-1]
            A[n-1] = A[i]
            A[i] = temp
        else:
            temp = A[n-1]
            A[n-1] = A[0]
            A[0] = temp
        permute(n-1, A)

def derangements(n):
    q = [0]
    p = []
    for n in range(1, m+1):
        sum = 0
        for k in range(1, n+1):
            a = math.comb(n, k)
            b = math.factorial(n-k) - q[n-k]
            sum += (a * b)

        q.append(sum)
        p.append(sum/float(math.factorial(n)))

    print(q)
    print(p)

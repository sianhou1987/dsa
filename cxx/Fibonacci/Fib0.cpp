//计算Fibonacci数列的第n项（迭代版）：O(n)
long long fibI(int n)
{
    long f = 1, g = 0;
    while (0 < n--)
    {
        g = g + f;
        f = g - f;
    }
    return g;
}
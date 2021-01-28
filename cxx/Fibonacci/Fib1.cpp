//计算Fibonacci数列的第n项（二分递归版）：O(2^n)
long long fib(int n)
{
    return (2 > n) ? (long long)n : fib(n - 1) + fib(n - 2);
}
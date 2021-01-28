//计算Fibonacci数列第n项（线性递归版）：入口形式fib(n, prev)
long long fib(int n, long long &prev)
{
    if (0 == n)
    {
        prev = 1;
        return 0;
    }
    else
    {
        long long prevPrev; prev = fib(n - 1, prevPrev);
        return prevPrev + prev;
    }
}//用辅助变量记录前一项，返回数列的当前项，O(n)
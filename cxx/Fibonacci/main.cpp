#include <time.h>
#include <iostream>
using namespace std;

#include "include/Fib.h"

long long fibI(int n);
long long fib(int n);
long long fib(int n, long long &prev);

int main(int argc, char *argv[])
{
    // 检查参数
    if (2 > argc)
    {
        fprintf(stderr, "Usage: %s <rank>\n", argv[0]);
        return 1;
    }
    int n = atoi(argv[1]);
    // 依次计算Fibonacci数列各项
    printf("\n------------- class Fib -------------\n");
    Fib f(0);
    for (int i = 0; i < n; i++, f.next())
        printf("fib(%2d) = %22I64d\n", i, f.get());
    for (int i = 0; i <= n; i++, f.prev())
        printf("fib(%2d) = %22I64d\n", n - i, f.get());
    printf("\n------------- Iteration -------------\n");
    for (int i = 0; i < n; i++)
        printf("fib(%2d) = %22I64d\n", i, fibI(i));
    printf("\n------------- Linear Recursion -------------\n");
    for (int i = 0; i < n; i++)
    {
        long long f;
        printf("fib(%2d) = %22I64d\n", i, fib(i, f));
    }
    printf("\n------------- Binary Recursion -------------\n");
    for (int i = 0; i < n; i++)
        printf("fib(%2d) = %22I64d\n", i, fib(i));
    return 0;
}
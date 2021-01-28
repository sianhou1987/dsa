#pragma once

class Fib
{
private:
    int f, g;
public:
    Fib(int n)
    {
        f = 1;
        g = 0;
        while (g < n)
            next();
    };
    ~Fib(){};
    int get() { return g; }
    int next()
    {
        g = g + f;
        f = g - f;
        return g;
    }
    int prev()
    {
        f = g - f;
        g = g - f;
        return g;
    }
};

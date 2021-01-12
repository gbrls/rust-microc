int fib(int n)
{
    int ans;
    if (n < 2)
    {
        ans = n;
    }
    else
    {
        ans = fib(n - 1) + fib(n - 2);
    }

    ans;
}

int x;
int _start()
{
    x = fib(13);
}
x;
// more than 13 it overflows

int fib(int n)
{
    int ans;
    if (n < 2)
    {
        ans = n;
    }
    else
    {
        int a;
        int b;

        a = fib(n - 1);
        b = fib(n - 2);

        ans = a + b;
    }

    ans;
}

int x;
int main()
{
    x = fib(13);
}
x;
// more than 13 it overflows

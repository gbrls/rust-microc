int y;

int max(int a, int b)
{
    int ans;
    if (a < b)
    {
        ans = b;
    }
    else
    {
        ans = a;
    }

    ans;
}

int mmax(int a, int b, int c)
{
    max(max(a, b), c);
}

int main()
{
    int z;
    z = 9;
    if (z < 10)
    {
        y = max(10, 3);
    }
    else
    {
        y = max(2, 13);
    }

    y = y + z;

    y = y + mmax(1, 2, 3);
}
y;
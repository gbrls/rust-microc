// 2^6 < INT_MAX < 2^7
int ans;
{
    int a;
    a = 0;
    ans = 1;

    while (a < 50 && ans < 64)
    {
        ans = ans * 2;
        a = a + 1;
    }
}
ans;
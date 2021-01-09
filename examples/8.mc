// 2^7 < INT_MAX < 2^8
int ans;
{
    ans = 1;

    while (ans < 64)
    {
        ans = ans * 2;
    }
}
ans;
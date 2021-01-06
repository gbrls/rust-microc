int a;
{
    a = 10;
    if (false)
    {
        a = 1;
        if (false)
        {
            a = 2;
        }
        a = 5;
    }
}
a;
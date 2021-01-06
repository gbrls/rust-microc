int a;
{
    a = 0;
    if (false)
    {
        a = 1;
    }
    else
    {
        a = 2;
        if (true)
        {
            a = 3;
        }
        else
        {
            a = 4;
        }
    }
}
a;
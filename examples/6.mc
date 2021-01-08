int a;
{
    a = 10;
    if (false)
    {
        a = 11;
    }
    else
    {
        a = 12;
        if (true)
        {
            a = 13;
        }
        else
        {
            a = 14;
        }
        a = 15;
    }
    a = 16;
}
a;
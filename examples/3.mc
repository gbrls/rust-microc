int b;
{
    b = 2;
    int a;
    a = 42;

    {
        int c;
        c = 1;
        a = a - c;
        a = a - 10 * b;
    }

    b = a;
}
b;
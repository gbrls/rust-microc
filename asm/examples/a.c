#include <stdio.h>

void fn(char *str)
{
    char *p = str;
    puts(p);
}

int global = 10;

int main()
{

    char str[32];
    scanf("%s", str);
    fn(str);

    char lit[] = "Literal";
    puts(lit);

    global = 5;
}

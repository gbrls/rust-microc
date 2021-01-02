int a;
bool b;
int c;

a = 10;
c = 2;

a = a - c;

// this will fail in compile time
// a = b + 1;

// this will fail in compile time
// a = d * c;

// this will fail in compile time
// d = a * c;
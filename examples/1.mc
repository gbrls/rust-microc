// TBH I have no idea what I was doing here and
// it is a miracle that this works.

// All variables are u8 (unsigned char)
// The output of the program will be
// the value of the last statement.

// variable assignment
a = 10
// precedence and integer division
b = (a * 2 + 5) / 5

// you can reference undeclared variables (bug or feature? bug!)
// all variables are initialized as 0 (feature!)
c = d
b = b + d

// same as exit(b) in C
b;

// THIS IS NO LONGER COMPATIBLE
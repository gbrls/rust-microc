nasm -felf64 internal.asm
ld internal.o
./a.out
echo $?

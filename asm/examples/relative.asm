global main


section .data

x dw 54 
y dw 63
z dw 72
hello db "Hello there, general kenobi", 0

section .text

extern puts

main:
xor ax, ax
mybp:

lea rdi, [rel hello]
call puts WRT ..plt

lea rcx, [rel x]
mov bx, [rcx]


mov eax, 1   
int 0x80     

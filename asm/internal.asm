global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

b dd 0
a dd 0

section .text

_start:

mov ax, 10
push ax
pop ax
mov [a], al
mov ax, 5
push ax
pop ax
mov [b], al
mov al, [a]
push ax
mov al, [b]
push ax
pop bx
pop ax
sub ax, bx
push ax
pop ax
mov [a], al
mov al, [a]
push ax

EXIT

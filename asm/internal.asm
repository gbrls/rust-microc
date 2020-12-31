global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data


section .text

_start:

mov ax, 6
push ax
mov ax, 1
push ax
pop bx
pop ax
sub ax, bx
push ax
mov ax, 2
push ax
pop bx
pop ax
div bx
push ax

EXIT

global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

a dd 0
b db 0
c dd 0

section .text

_start:

mov ax, 10
push ax
pop ax
mov [a], al
mov ax, 2
push ax
pop ax
mov [c], al
mov al, [a]
push ax
mov al, [c]
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

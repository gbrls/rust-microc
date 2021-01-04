global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

b db 0
c dd 0
a dd 0

section .text

_start:

mov rbp, rsp
mov ax, 10
push ax
pop ax
mov [a], ax
mov ax, 2
push ax
pop ax
mov [c], ax
mov ax, [a]
push ax
mov ax, [c]
push ax
pop bx
pop ax
sub ax, bx
push ax
pop ax
mov [a], ax
mov ax, [a]
push ax

EXIT

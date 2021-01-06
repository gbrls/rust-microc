global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

a dd 0

section .text

_start:

mov rbp, rsp
mov ax, 10
push ax
pop ax
mov [a], eax
mov ax, 0
push ax
test ax, ax
je .L1
mov ax, 1
push ax
pop ax
mov [a], eax
mov ax, 0
push ax
test ax, ax
je .L0
mov ax, 2
push ax
pop ax
mov [a], eax
add rsp, 0
.L0:
mov ax, 5
push ax
pop ax
mov [a], eax
add rsp, 0
.L1:
add rsp, 0
mov eax, [a]
push ax

EXIT

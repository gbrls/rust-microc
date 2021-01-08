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
mov ax, 1
push ax
test ax, ax
je .L1
mov ax, 0
push ax
test ax, ax
jne .L0
mov ax, 1
push ax
pop bx
pop ax
or eax, ebx
push ax
.L0:
pop bx
pop ax
and eax, ebx
push ax
.L1: ;; IF's test
pop ax
test ax, ax
je .L2
mov ax, 11
push ax
pop ax
mov [a], eax
add rsp, 0
jmp .L3
.L2:
.L3:
sub rsp, 4
add rsp, 4
mov eax, [a]
push ax

EXIT

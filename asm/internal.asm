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
mov ax, 0
push ax
pop ax
mov [a], eax
mov ax, 0
push ax
test ax, ax
je .L0
mov ax, 1
push ax
pop ax
mov [a], eax
add rsp, 0
jmp .L3
.L0:
mov ax, 2
push ax
pop ax
mov [a], eax
mov ax, 1
push ax
test ax, ax
je .L1
mov ax, 3
push ax
pop ax
mov [a], eax
add rsp, 0
jmp .L2
.L1:
mov ax, 4
push ax
pop ax
mov [a], eax
add rsp, 0
.L2:
add rsp, 0
.L3:
add rsp, 0
mov eax, [a]
push ax

EXIT

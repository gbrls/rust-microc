global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

ans dd 0

section .text

_start:

mov rbp, rsp
mov ax, 1
push ax
pop ax
mov [ans], eax
.L0:
mov eax, [ans]
push ax
mov ax, 64
push ax
pop bx
pop ax
cmp eax, ebx
mov bx, 1
mov cx, 0
cmovb ax, bx
cmovae ax, cx
push ax
test ax, ax
je .L1
mov eax, [ans]
push ax
mov ax, 2
push ax
pop bx
pop ax
mul ebx
push ax
pop ax
mov [ans], eax
add rsp, 0
jmp .L0
.L1:
add rsp, 0
mov eax, [ans]
push ax

EXIT

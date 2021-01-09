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
sub rsp, 4
mov ax, 0
push ax
pop ax
mov [rbp-4], eax
mov ax, 1
push ax
pop ax
mov [ans], eax
.L0:
mov eax, [rbp-4]
push ax
mov ax, 50
push ax
pop bx
pop ax
cmp eax, ebx
mov bx, 1
mov cx, 0
cmovb ax, bx
cmova ax, cx
push ax
test ax, ax
je .L1
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
cmova ax, cx
push ax
pop bx
pop ax
and eax, ebx
push ax
.L1:
test ax, ax
je .L2
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
mov eax, [rbp-4]
push ax
mov ax, 1
push ax
pop bx
pop ax
add eax, ebx
push ax
pop ax
mov [rbp-4], eax
add rsp, 0
jmp .L0
.L2:
add rsp, 4
mov eax, [ans]
push ax

EXIT

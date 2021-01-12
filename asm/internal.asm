global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

x dd 0

section .text

fib:
push rbp
mov rbp, rsp
sub rsp, 4
mov ax, [rbp+16]
push ax
mov ax, 2
push ax
pop bx
pop ax
cmp ax, bx
mov bx, 1
mov cx, 0
cmovb ax, bx
cmovae ax, cx
push ax
pop ax
test ax, ax
je .L0
mov ax, [rbp+16]
push ax
pop ax
mov [rbp-4], ax
add rsp, 0
jmp .L1
.L0:
mov ax, [rbp+16]
push ax
mov ax, 1
push ax
pop bx
pop ax
sub eax, ebx
push ax
call fib
add rsp, 2
push ax
mov ax, [rbp+16]
push ax
mov ax, 2
push ax
pop bx
pop ax
sub eax, ebx
push ax
call fib
add rsp, 2
push ax
pop bx
pop ax
add eax, ebx
push ax
pop ax
mov [rbp-4], ax
add rsp, 0
.L1:
mov ax, [rbp-4]
push ax
pop ax
add rsp, 4
pop rbp
ret

_start:
push rbp
mov rbp, rsp
mov ax, 13
push ax
call fib
add rsp, 2
push ax
pop ax
mov [x], ax
pop ax
add rsp, 0
pop rbp
mov ax, [x]
push ax

EXIT

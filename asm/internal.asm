global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

b dd 0

section .text

_start:

mov rbp, rsp
mov ax, 2
push ax
pop ax
mov [b], ax
sub rsp, 4
mov ax, 42
push ax
pop ax
mov [rbp-4], ax
sub rsp, 4
mov ax, 1
push ax
pop ax
mov [rbp-8], ax
mov ax, [rbp-4]
push ax
mov ax, [rbp-8]
push ax
pop bx
pop ax
sub ax, bx
push ax
pop ax
mov [rbp-4], ax
mov ax, [rbp-4]
push ax
mov ax, 10
push ax
mov ax, [b]
push ax
pop bx
pop ax
mul bx
push ax
pop bx
pop ax
sub ax, bx
push ax
pop ax
mov [rbp-4], ax
add rsp, 4
mov ax, [rbp-4]
push ax
pop ax
mov [b], ax
add rsp, 4
mov ax, [b]
push ax

EXIT

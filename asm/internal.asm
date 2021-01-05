global _start
%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

t db 0

section .text

_start:

mov rbp, rsp
sub rsp, 4
mov ax, 32
push ax
pop ax
mov [rbp-4], eax
add rsp, 4
mov ax, 0
push ax
pop ax
mov [t], al
mov al, [t]
push ax
pop ax
mov [t], al
add rsp, 0
mov al, [t]
push ax

EXIT

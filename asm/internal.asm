%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

section .data

a db 0
y db 0

section .text

_start:

mov ax, 10
push ax
pop ax
mov [a], al
mov al, [a]
push ax
mov al, [a]
push ax
pop bx
pop ax
mul bx
push ax
pop ax
mov [a], al
mov al, [a]
push ax
mov ax, 20
push ax
pop bx
pop ax
div bx
push ax
mov ax, 1
push ax
pop bx
pop ax
sub ax, bx
push ax
pop ax
mov [y], al
mov al, [y]
push ax
EXIT

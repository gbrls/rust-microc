global _start
section .data
; data will go here
section .text

%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

_start:
mov ax, 1
push ax
mov ax, 2
push ax
pop bx
pop ax
add ax, bx
push ax
mov ax, 3
push ax
pop bx
pop ax
mul bx
push ax
mov ax, 2
push ax
pop bx
pop ax
add ax, bx
push ax
mov ax, 4
push ax
pop bx
pop ax
mul bx
push ax
EXIT

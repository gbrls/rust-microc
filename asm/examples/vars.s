%macro EXIT 0
mov eax, 1   ; 1 stands for EXIT syscall
pop bx       ; pop the status code
int 0x80     ; make system call
%endmacro

global _start
section .data

; DB (define byte)
; DW (define word)
; DD (define double word)
_globals:

x db 0
y db 0
z db 0

section .text

_start:

mov bl, 8
mov [y], bl
mov al, [y]
push ax

EXIT

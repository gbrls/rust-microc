global main
extern printf
extern puts 
%macro EXIT 0
mov bx, ax
mov eax, 1   ; 1 stands for EXIT syscall
int 0x80     ; make system call
%endmacro

%macro PRINT64 0
pop bx

; Arguments:
; rdi, rsi, rdx, rcx, r8, r9
; https://stackoverflow.com/questions/2535989/what-are-the-calling-conventions-for-unix-linux-system-calls-and-user-space-f

lea rdi, [rel pstr64]
mov rsi, rbx

xor eax, eax ; https://stackoverflow.com/questions/6212665/why-is-eax-zeroed-before-a-call-to-printf
call printf WRT ..plt
%endmacro

%macro HELLO 0
lea rdi, [rel hellostr]
call puts WRT ..plt
%endmacro

section .data

pstr64 db "%hu",10,0
hellostr db "MicroC :)",0

section .text

main:
push rbp
mov rbp, rsp
sub rsp, 4
sub rsp, 4
mov ax, 65536
push ax
mov ax, 1
push ax
pop bx
pop ax
sub eax, ebx
push ax
pop ax
mov [rbp-8], ax
mov ax, [rbp-8]
push ax
pop ax
mov [rbp-4], ax
.L0:
mov ax, 0
push ax
mov ax, [rbp-4]
push ax
pop bx
pop ax
cmp ax, bx
mov bx, 1
mov cx, 0
cmovb ax, bx
cmovae ax, cx
push ax
test ax, ax
je .L1
mov ax, [rbp-4]
push ax
PRINT64
add rsp, 2
push ax
mov ax, [rbp-4]
push ax
mov ax, 1
push ax
pop bx
pop ax
sub eax, ebx
push ax
pop ax
mov [rbp-4], ax
add rsp, 0
jmp .L0
.L1:
mov ax, [rbp-4]
push ax
pop ax
add rsp, 8
pop rbp

EXIT

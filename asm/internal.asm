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

push ax
xor eax, eax ; https://stackoverflow.com/questions/6212665/why-is-eax-zeroed-before-a-call-to-printf
call printf WRT ..plt
pop ax
%endmacro

%macro HELLO 0
lea rdi, [rel hellostr]
call puts WRT ..plt
%endmacro

section .data

x dd 0
pstr64 db "%hu",10,0
hellostr db "MicroC :)",0

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
sub rsp, 4
sub rsp, 4
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
pop ax
mov [rbp-8], ax
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
pop ax
mov [rbp-12], ax
mov ax, [rbp-8]
push ax
mov ax, [rbp-12]
push ax
pop bx
pop ax
add eax, ebx
push ax
pop ax
mov [rbp-4], ax
add rsp, 8
.L1:
mov ax, [rbp-4]
push ax
PRINT64
add rsp, 2
push ax
mov ax, [rbp-4]
push ax
pop ax
add rsp, 4
pop rbp
ret

main:

xor rax, rax
xor rbx, rbx
xor rcx, rcx
xor rdx, rdx

push rbp
mov rbp, rsp
mov ax, 0
push ax
pop ax
mov [rel x], ax
mov ax, 23
push ax
call fib
add rsp, 2
push ax
PRINT64
add rsp, 2
push ax
pop ax
add rsp, 0
pop rbp
mov ax, [rel x]
push ax

EXIT

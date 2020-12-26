global main
section .text

main:
    mov eax, 1   ; use the `_exit` [interrupt-flavor] system call
    mov ebx, 0x20   ; error code 0
    int 0x80       ; make system call

    ret
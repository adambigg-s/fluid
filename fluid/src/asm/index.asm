section .data
x dq 0
y dq 0
dx dq 0
dy dq 0
nx dq 0
ny dq 0

section .text
global _start

_start:
mov rax, [x]
add rax, [dx]
mov [nx], rax

mov rbx, [y]
add rbx, [dy]
mov [ny], rbx

mov eax, 1
xor ebx, ebx
int 0x80


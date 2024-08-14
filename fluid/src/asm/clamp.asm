section .data
value dq 0.0
min dq 0.0
max dq 0.0
result dq 0.0

section .text
global _start

_start:
movss xmm0, [value]
movss xmm1, [min]
movss xmm2, [max]

maxss xmm0, xmm1
minss xmm0, xmm2

movss [result], xmm0

mov eax, 1
xor ebx, ebx
int 0x80


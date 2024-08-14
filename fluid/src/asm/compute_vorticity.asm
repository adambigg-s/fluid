section .data
u_plus1_j dq 0.0
u_minus1_j dq 0.0
v_i_jplus1 dq 0.0
v_i_jminus1 dq 0.0
half dq 0.0
vorticity_ij dq 0.0
_dwdy dq 0.0
_dudx dq 0.0

section .text
global _start

_start:
movss xmm0, [u_plus1_j]
movss xmm1, [u_minus1_j]
subss xmm0, xmm1
mulss xmm0, [half]
movss [_dwdy], xmm0

movss xmm2, [v_i_jplus1]
movss xmm3, [v_i_jminus1]
subss xmm2, xmm3
mulss xmm2, [half]
movss [_dudx], xmm2
subss xmm0, xmm2

movss [vorticity_ij], xmm0

mov eax, 1
xor ebx, ebx
int 0x80


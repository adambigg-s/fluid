section .data
gwx    dq 0.0
gwy    dq 0.0
mag    dq 0.0
vort   dq 0.0
epsilon dq 0.0
fx     dq 0.0
fy     dq 0.0

section .text
global _start

_start:
movss xmm0, [gwx]
movss xmm1, [gwy]
movss xmm2, [mag]
movss xmm3, [vort]
divss xmm0, xmm2
divss xmm1, xmm2
mulss xmm0, xmm3
mulss xmm1, xmm3
mulss xmm0, [epsilon]
mulss xmm1, [epsilon]
movss [fx], xmm0
movss [fy], xmm1
mov eax, 1
xor ebx, ebx
int 0x80

section .data
vorticity1 dq 0.0
vorticity2 dq 0.0
vorticity3 dq 0.0
vorticity4 dq 0.0
half dq 0.0
grad_w_x dq 0.0
grad_w_y dq 0.0

section .text
global _start

_start:
movss xmm0, [vorticity1]
movss xmm1, [vorticity2]
subss xmm0, xmm1
mulss xmm0, [half]
movss [grad_w_x], xmm0

movss xmm1, [vorticity3]
movss xmm2, [vorticity4]
subss xmm1, xmm2
mulss xmm1, [half]
movss [grad_w_y], xmm1

mov eax, 1
xor ebx, ebx
int 0x80


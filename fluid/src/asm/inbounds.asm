section .data
x dq 0
y dq 0
bx dq 0
by dq 0
result db 0

section .text
global _start

_start:
cmp [x], [bx]
jae label1
cmp [y], [by]
jae label1
mov [result], 1
jmp end
label1:
mov [result], 0
end:

mov eax, 1
xor ebx, ebx
int 0x80


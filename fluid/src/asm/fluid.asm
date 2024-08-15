section .data

section .bss

section .text

global oo_construct, oo_set_here, oo_remove_here, oo_peek_element_here, oo_peek_velocity
global oo_peek_velocity_mut, oo_divergence_here, oo_modify_adjacent, oo_set_velocity_polarized
global oo_set_velocity_zeros, oo_set_velocity_matched, oo_afflicted_area, oo_index

oo_construct:
    push rbx
    mov rbx, rdi
    mov rcx, rsi
    mov rdx, rdx
    mov rdi, rbx
    mov rsi, rcx
    mov rdx, rdx
    pop rbx
    ret

oo_set_here:
    mov rbx, [rdi + 16]
    mov rdi, [rbx + rsi*8 + rdx*4]
    mov [rdi], rdx
    ret

oo_remove_here:
    mov rbx, [rdi + 16]
    mov rdi, [rbx + rsi*8 + rdx*4]
    mov byte [rdi], 0
    ret

oo_peek_element_here:
    call oo_index
    mov rbx, [rdi + 16]
    mov rdi, [rbx + rsi*8 + rdx*4]
    ret

oo_peek_velocity:
    call oo_index
    mov rbx, [rdi + 16]
    mov rdi, [rbx + rsi*8 + rdx*4]
    ret

oo_peek_velocity_mut:
    call oo_index
    mov rbx, [rdi + 16]
    mov rdi, [rbx + rsi*8 + rdx*4]
    ret

oo_divergence_here:
    call oo_peek_velocity
    sub xmm0, xmm1
    call oo_peek_velocity
    sub xmm0, xmm1
    ret

oo_modify_adjacent:
    call oo_peek_element_here
    test al, al
    jz .skip
    call oo_peek_velocity_mut
    addss xmm0, xmm1
.skip:
    ret

oo_set_velocity_polarized:
    call oo_peek_velocity_mut
    movss [rdi], xmm0
    call oo_peek_velocity_mut
    movss [rdi], xmm1
    ret

oo_set_velocity_zeros:
    xorps xmm0, xmm0
    call oo_peek_velocity_mut
    movss [rdi], xmm0
    ret

oo_set_velocity_matched:
    call oo_peek_velocity
    movss xmm0, xmm1
    call oo_peek_velocity
    movss xmm0, xmm1
    ret

oo_afflicted_area:
    xorps xmm0, xmm0
    call oo_peek_element_here
    addss xmm0, xmm1
    ret

oo_index:
    mov rax, rsi
    add rax, rdx
    mov rbx, rdi
    add rbx, rcx
    ret

section .data
    SCALE_FACTOR dq 32
    WIDTH dq 80 * 32
    HEIGHT dq 20 * 32
    CELL_SIZE dq 35.0 / 32.0
    OVERRELAXATION dq 1.97
    ITERS dq 150
    DELTA_T dq 0.1
    SOURCE_V dq 80.0
    VISUAL_MOD dq 2.0
    GRID_SIZE dq 2.0
    VORT_CONF_EPSILON dq 0.5

section .bss
    x resq 1
    y resq 1
    overrelaxation resq 1
    cell_size resq 1
    iters resq 1
    delta_t resq 1
    source_velocity resq 1
    visual_modifier resq 1
    grid_size resq 1
    epsilon resq 1

section .text
    global _start

    _start:
        mov rax, WIDTH
        mov [x], rax

        mov rax, HEIGHT
        mov [y], rax

        mov rax, OVERRELAXATION
        mov [overrelaxation], rax

        mov rax, CELL_SIZE
        mov [cell_size], rax

        mov rax, ITERS
        mov [iters], rax

        mov rax, DELTA_T
        mov [delta_t], rax

        mov rax, SOURCE_V
        mov [source_velocity], rax

        mov rax, VISUAL_MOD
        mov [visual_modifier], rax

        mov rax, GRID_SIZE
        mov [grid_size], rax

        mov rax, VORT_CONF_EPSILON
        mov [epsilon], rax

        mov rdi,

        mov rax, 0 

        mov rax, 0

        mov rax, 60
        xor rdi, rdi
        syscall


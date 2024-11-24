.global _start
.extern _printf
.extern _exit

.section __DATA,__data
msg:.asciz "Hello, world!\n"

.section __TEXT,__text

_start:
    adrp x0, msg@PAGE@PAGE
    add x0, x0, msg@PAGEOFF@PAGEOFF
    bl _printf
    mov x0, #0
    bl _exit

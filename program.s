// cat program.s && as -o program.o program.s -arch arm64 --target=arm64-apple-macos11.0 && ld -o program program.o -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib -lc -lSystem -macos_version_min 11.0 -e _start && ./program
.global _start         // Define the global entry point
.extern _printf        // Declare the external function
.extern _exit          // Declare the _exit function (underscore required for macOS)

.section __DATA,__data
msg:    .asciz "Hello, world!\n"  // Define a null-terminated string

.section __TEXT,__text
_start:
    adrp x0, msg@PAGE         // Load the page of 'msg' into x0
    add x0, x0, msg@PAGEOFF   // Add the page offset of 'msg' to x0
    bl _printf                // Call printf

    // Exit the program using _exit
    mov x0, #0                // Exit status 0
    bl _exit                  // Call _exit


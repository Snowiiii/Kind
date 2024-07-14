nasm -felf64 test.asm
ld test.o -o test
./test
echo $?
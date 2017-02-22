all: dist/FanmiOS.img

run: dist/FanmiOS.img
	qemu-system-i386.exe -drive file=.\dist\FanmiOS.img,if=floppy,format=raw

dev: all run

build/main.o: src/main.rs
	rustc -O --target i686-unknown-linux-gnu --crate-type lib -o $@ --emit obj $<

build/main.bin: build/main.o
	bash -e -c "ld -m elf_i386 -o $@ -T linker.ld $<"

build/loader.bin: src/loader.asm
	nasm -o $@ -f bin $<

dist/FanmiOS.img: build/loader.bin build/main.bin
	powershell -command "Get-Content build/loader.bin,build/main.bin -Encoding Byte | Set-Content $@ -Encoding Byte"

clean:
	del .\build\*.o
	del .\build\*.bin
	del .\dist\*.img
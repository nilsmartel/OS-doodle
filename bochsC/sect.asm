loop:
	jmp loop  ; infinite loop

times 510-($-$$) db 0 ; pad our programm with 510 zero bytes

dw 0xaa55; 	write last 2 bytes

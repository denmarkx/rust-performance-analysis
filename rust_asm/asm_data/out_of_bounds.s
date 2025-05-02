	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.intel_syntax noprefix
	.file	"out_of_bounds.685f15ca9aea7ae2-cgu.0"
	.def	_ZN13out_of_bounds5check17ha6502a09a1e86cb6E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN13out_of_bounds5check17ha6502a09a1e86cb6E
	.globl	_ZN13out_of_bounds5check17ha6502a09a1e86cb6E
	.p2align	4, 0x90
_ZN13out_of_bounds5check17ha6502a09a1e86cb6E:
.seh_proc _ZN13out_of_bounds5check17ha6502a09a1e86cb6E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	cmp	qword ptr [rcx + 16], 0
	je	.LBB0_2
	add	rsp, 40
	ret
.LBB0_2:
	lea	r8, [rip + __unnamed_1]
	xor	ecx, ecx
	xor	edx, edx
	call	_ZN4core9panicking18panic_bounds_check17h883675bd61368af7E
	int3
	.seh_endproc

	.section	.rdata,"dr",one_only,__unnamed_2
__unnamed_2:
	.ascii	"out_of_bounds.rs"

	.section	.rdata,"dr",one_only,__unnamed_1
	.p2align	3, 0x0
__unnamed_1:
	.quad	__unnamed_2
	.asciz	"\020\000\000\000\000\000\000\000\r\000\000\000\023\000\000"


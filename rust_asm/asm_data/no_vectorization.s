	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.intel_syntax noprefix
	.file	"auto_vectorization.be89dbe02c20b45b-cgu.0"
	.def	_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E
	.globl	_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E
	.p2align	4, 0x90
_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E:
.seh_proc _ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	r8, qword ptr [rcx + 16]
	xor	eax, eax
	cmp	r8, 2
	jb	.LBB0_3
	mov	rax, qword ptr [rdx + 16]
	lea	r9, [r8 - 2]
	cmp	rax, r9
	jbe	.LBB0_4
	mov	rax, qword ptr [rdx + 8]
	mov	rcx, qword ptr [rcx + 8]
	mov	eax, dword ptr [rax + 4*r8 - 8]
	imul	eax, dword ptr [rcx + 4*r8 - 4]
.LBB0_3:
	add	rsp, 40
	ret
.LBB0_4:
	lea	r8, [rip + __unnamed_1]
	mov	rcx, rax
	mov	rdx, rax
	call	_ZN4core9panicking18panic_bounds_check17h883675bd61368af7E
	int3
	.seh_endproc

	.section	.rdata,"dr",one_only,__unnamed_2
__unnamed_2:
	.ascii	"auto_vectorization.rs"

	.section	.rdata,"dr",one_only,__unnamed_1
	.p2align	3, 0x0
__unnamed_1:
	.quad	__unnamed_2
	.asciz	"\025\000\000\000\000\000\000\000\t\000\000\000\034\000\000"


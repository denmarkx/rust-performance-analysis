	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.intel_syntax noprefix
	.file	"auto_vectorization.4b83c04384182820-cgu.0"
	.def	_ZN18auto_vectorization6update17haf70a013dfeb101bE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN18auto_vectorization6update17haf70a013dfeb101bE
	.globl	_ZN18auto_vectorization6update17haf70a013dfeb101bE
	.p2align	4
_ZN18auto_vectorization6update17haf70a013dfeb101bE:
.seh_proc _ZN18auto_vectorization6update17haf70a013dfeb101bE
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 32
	.seh_stackalloc 32
	.seh_endprologue
	mov	r8, rdx
	mov	rdx, qword ptr [rcx + 16]
	xor	eax, eax
	cmp	rdx, 4
	jb	.LBB0_5
	mov	r11, rdx
	shr	r11
	mov	rsi, qword ptr [r8 + 8]
	mov	r9, qword ptr [r8 + 16]
	mov	r8, qword ptr [rcx + 8]
	lea	rdi, [rdx - 1]
	shr	rdi
	dec	r11
	xor	ecx, ecx
	mov	r10d, 2
	xor	eax, eax
	.p2align	4
.LBB0_2:
	cmp	r9, rcx
	je	.LBB0_6
	cmp	rdi, rcx
	je	.LBB0_7
	mov	ebx, dword ptr [rsi + 4*rcx]
	imul	ebx, dword ptr [r8 + 4*rcx + 4]
	add	ebx, ebx
	add	dword ptr [r8 + 8*rcx + 8], ebx
	add	eax, dword ptr [r8 + 4*rcx + 4]
	inc	rcx
	add	r10, 2
	cmp	r11, rcx
	jne	.LBB0_2
.LBB0_5:
	add	rsp, 32
	pop	rbx
	pop	rdi
	pop	rsi
	ret
.LBB0_6:
	lea	r8, [rip + anon.271030ac748f6b252f0b1933c629d6fc.1]
	mov	rdx, r9
	call	_ZN4core9panicking18panic_bounds_check17h3ead85185f0acabaE
.LBB0_7:
	lea	r8, [rip + anon.271030ac748f6b252f0b1933c629d6fc.2]
	mov	rcx, r10
	call	_ZN4core9panicking18panic_bounds_check17h3ead85185f0acabaE
	int3
	.seh_endproc

	.section	.rdata,"dr",one_only,anon.271030ac748f6b252f0b1933c629d6fc.0
anon.271030ac748f6b252f0b1933c629d6fc.0:
	.ascii	"auto_vectorization.rs"

	.section	.rdata,"dr",one_only,anon.271030ac748f6b252f0b1933c629d6fc.1
	.p2align	3, 0x0
anon.271030ac748f6b252f0b1933c629d6fc.1:
	.quad	anon.271030ac748f6b252f0b1933c629d6fc.0
	.asciz	"\025\000\000\000\000\000\000\000\r\000\000\000\033\000\000"

	.section	.rdata,"dr",one_only,anon.271030ac748f6b252f0b1933c629d6fc.2
	.p2align	3, 0x0
anon.271030ac748f6b252f0b1933c629d6fc.2:
	.quad	anon.271030ac748f6b252f0b1933c629d6fc.0
	.asciz	"\025\000\000\000\000\000\000\000\r\000\000\000\r\000\000"


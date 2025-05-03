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
	.globl	__xmm@00000000000000030000000000000002
	.section	.rdata,"dr",discard,__xmm@00000000000000030000000000000002
	.p2align	4, 0x0
__xmm@00000000000000030000000000000002:
	.quad	2
	.quad	3
	.globl	__xmm@00000000000000050000000000000004
	.section	.rdata,"dr",discard,__xmm@00000000000000050000000000000004
	.p2align	4, 0x0
__xmm@00000000000000050000000000000004:
	.quad	4
	.quad	5
	.globl	__xmm@00000000000000070000000000000006
	.section	.rdata,"dr",discard,__xmm@00000000000000070000000000000006
	.p2align	4, 0x0
__xmm@00000000000000070000000000000006:
	.quad	6
	.quad	7
	.globl	__xmm@80000000800000008000000080000000
	.section	.rdata,"dr",discard,__xmm@80000000800000008000000080000000
	.p2align	4, 0x0
__xmm@80000000800000008000000080000000:
	.quad	-9223372034707292160
	.quad	-9223372034707292160
	.section	.text,"xr",one_only,_ZN18auto_vectorization6update17haf70a013dfeb101bE
	.globl	_ZN18auto_vectorization6update17haf70a013dfeb101bE
	.p2align	4
_ZN18auto_vectorization6update17haf70a013dfeb101bE:
.seh_proc _ZN18auto_vectorization6update17haf70a013dfeb101bE
	push	r14
	.seh_pushreg r14
	push	rsi
	.seh_pushreg rsi
	push	rdi
	.seh_pushreg rdi
	push	rbp
	.seh_pushreg rbp
	push	rbx
	.seh_pushreg rbx
	sub	rsp, 224
	.seh_stackalloc 224
	movdqa	xmmword ptr [rsp + 208], xmm15
	.seh_savexmm xmm15, 208
	movdqa	xmmword ptr [rsp + 192], xmm14
	.seh_savexmm xmm14, 192
	movaps	xmmword ptr [rsp + 176], xmm13
	.seh_savexmm xmm13, 176
	movdqa	xmmword ptr [rsp + 160], xmm12
	.seh_savexmm xmm12, 160
	movdqa	xmmword ptr [rsp + 144], xmm11
	.seh_savexmm xmm11, 144
	movdqa	xmmword ptr [rsp + 128], xmm10
	.seh_savexmm xmm10, 128
	movdqa	xmmword ptr [rsp + 112], xmm9
	.seh_savexmm xmm9, 112
	movdqa	xmmword ptr [rsp + 96], xmm8
	.seh_savexmm xmm8, 96
	movdqa	xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movdqa	xmmword ptr [rsp + 64], xmm6
	.seh_savexmm xmm6, 64
	.seh_endprologue
	mov	r9, rdx
	mov	r8, rcx
	mov	rdx, qword ptr [rcx + 16]
	mov	r11, rdx
	shr	r11
	movabs	r10, 1152921504606846968
	and	r10, r11
	cmp	r10, 9
	jae	.LBB0_6
	xor	eax, eax
	mov	ecx, 1
.LBB0_2:
	cmp	rcx, r11
	jae	.LBB0_57
	mov	rsi, qword ptr [r9 + 8]
	mov	r9, qword ptr [r9 + 16]
	mov	r8, qword ptr [r8 + 8]
	lea	r10, [rcx - 1]
	add	rcx, rcx
	neg	r11
	.p2align	4
.LBB0_4:
	cmp	r10, r9
	jae	.LBB0_5
	cmp	rcx, rdx
	jae	.LBB0_59
	mov	edi, dword ptr [rsi + 4*r10]
	imul	edi, dword ptr [r8 + 4*r10 + 4]
	add	edi, edi
	add	dword ptr [r8 + 8*r10 + 8], edi
	add	eax, dword ptr [r8 + 4*r10 + 4]
	add	rcx, 2
	lea	rdi, [r11 + r10]
	inc	rdi
	inc	r10
	cmp	rdi, -1
	jne	.LBB0_4
.LBB0_57:
	movaps	xmm6, xmmword ptr [rsp + 64]
	movaps	xmm7, xmmword ptr [rsp + 80]
	movaps	xmm8, xmmword ptr [rsp + 96]
	movaps	xmm9, xmmword ptr [rsp + 112]
	movaps	xmm10, xmmword ptr [rsp + 128]
	movaps	xmm11, xmmword ptr [rsp + 144]
	movaps	xmm12, xmmword ptr [rsp + 160]
	movaps	xmm13, xmmword ptr [rsp + 176]
	movaps	xmm14, xmmword ptr [rsp + 192]
	movaps	xmm15, xmmword ptr [rsp + 208]
	add	rsp, 224
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r14
	ret
.LBB0_6:
	mov	rsi, qword ptr [r8 + 8]
	movq	xmm0, rdx
	pshufd	xmm0, xmm0, 68
	movq	xmm1, rsi
	pshufd	xmm1, xmm1, 68
	movq	xmm2, qword ptr [r9 + 16]
	pshufd	xmm2, xmm2, 68
	movq	xmm3, qword ptr [r9 + 8]
	pshufd	xmm7, xmm3, 68
	xor	eax, eax
	mov	ebx, 9
	mov	edi, 4
	movdqa	xmm3, xmmword ptr [rip + __xmm@80000000800000008000000080000000]
	pxor	xmm0, xmm3
	movdqa	xmmword ptr [rsp + 32], xmm0
	pxor	xmm2, xmm3
	movdqa	xmmword ptr [rsp + 48], xmm1
	jmp	.LBB0_7
	.p2align	4
.LBB0_54:
	paddd	xmm12, xmm13
	pshufd	xmm1, xmm12, 238
	paddd	xmm1, xmm12
	pshufd	xmm3, xmm1, 85
	paddd	xmm3, xmm1
	movd	ebx, xmm3
	add	eax, ebx
	lea	rbx, [rcx + 8]
	add	rdi, 16
	cmp	rbx, r10
	movdqa	xmm1, xmmword ptr [rsp + 48]
	ja	.LBB0_2
.LBB0_7:
	mov	rcx, rbx
	add	rbx, -8
	lea	r14, [rcx - 7]
	movq	xmm8, rbx
	pshufd	xmm9, xmm8, 68
	movdqa	xmm6, xmm9
	paddq	xmm6, xmmword ptr [rip + __xmm@00000000000000030000000000000002]
	movdqa	xmm4, xmm9
	paddq	xmm4, xmmword ptr [rip + __xmm@00000000000000050000000000000004]
	paddq	xmm9, xmmword ptr [rip + __xmm@00000000000000070000000000000006]
	movq	xmm5, r14
	punpcklqdq	xmm8, xmm5
	movdqa	xmm5, xmm8
	psllq	xmm5, 2
	paddq	xmm5, xmm1
	movdqa	xmm10, xmm9
	movdqa	xmm3, xmmword ptr [rip + __xmm@80000000800000008000000080000000]
	pxor	xmm10, xmm3
	movdqa	xmm0, xmmword ptr [rsp + 32]
	movdqa	xmm11, xmm0
	pcmpgtd	xmm11, xmm10
	pshufd	xmm12, xmm11, 160
	pcmpeqd	xmm10, xmm0
	pshufd	xmm10, xmm10, 245
	pand	xmm10, xmm12
	pshufd	xmm11, xmm11, 245
	por	xmm11, xmm10
	movdqa	xmm10, xmm4
	pxor	xmm10, xmm3
	movdqa	xmm12, xmm0
	pcmpgtd	xmm12, xmm10
	pshufd	xmm13, xmm12, 160
	pcmpeqd	xmm10, xmm0
	pshufd	xmm14, xmm10, 245
	pand	xmm14, xmm13
	pshufd	xmm10, xmm12, 245
	por	xmm10, xmm14
	packssdw	xmm10, xmm11
	movdqa	xmm11, xmm8
	pxor	xmm11, xmm3
	movdqa	xmm12, xmm0
	pcmpgtd	xmm12, xmm11
	pshufd	xmm13, xmm12, 160
	pcmpeqd	xmm11, xmm0
	pshufd	xmm11, xmm11, 245
	pand	xmm11, xmm13
	pshufd	xmm12, xmm12, 245
	por	xmm12, xmm11
	movdqa	xmm11, xmm6
	pxor	xmm11, xmm3
	movdqa	xmm13, xmm0
	pcmpgtd	xmm13, xmm11
	pshufd	xmm14, xmm13, 160
	pcmpeqd	xmm11, xmm0
	pshufd	xmm11, xmm11, 245
	pand	xmm11, xmm14
	pshufd	xmm13, xmm13, 245
	por	xmm13, xmm11
	packssdw	xmm12, xmm13
	packssdw	xmm12, xmm10
	packsswb	xmm12, xmm12
	pmovmskb	ebx, xmm12
	pxor	xmm12, xmm12
	pxor	xmm13, xmm13
	test	bl, 1
	je	.LBB0_9
	movq	r14, xmm5
	movd	xmm12, dword ptr [r14]
.LBB0_9:
	test	bl, 2
	je	.LBB0_11
	pshufd	xmm5, xmm5, 238
	movq	r14, xmm5
	movd	xmm5, dword ptr [r14]
	punpcklqdq	xmm5, xmm12
	shufps	xmm5, xmm12, 226
	movaps	xmm12, xmm5
.LBB0_11:
	movdqa	xmm5, xmm6
	psllq	xmm5, 2
	paddq	xmm5, xmm1
	test	bl, 4
	je	.LBB0_13
	movq	r14, xmm5
	movss	xmm10, dword ptr [r14]
	shufps	xmm10, xmm12, 48
	shufps	xmm12, xmm10, 132
.LBB0_13:
	test	bl, 8
	je	.LBB0_15
	pshufd	xmm5, xmm5, 238
	movq	r14, xmm5
	movss	xmm5, dword ptr [r14]
	shufps	xmm5, xmm12, 228
	shufps	xmm12, xmm5, 36
.LBB0_15:
	movdqa	xmm5, xmm4
	psllq	xmm5, 2
	paddq	xmm5, xmm1
	test	bl, 16
	je	.LBB0_17
	movq	r14, xmm5
	movss	xmm10, dword ptr [r14]
	movss	xmm13, xmm10
.LBB0_17:
	test	bl, 32
	je	.LBB0_19
	pshufd	xmm5, xmm5, 238
	movq	r14, xmm5
	movss	xmm5, dword ptr [r14]
	movlhps	xmm5, xmm13
	shufps	xmm5, xmm13, 226
	movaps	xmm13, xmm5
.LBB0_19:
	movdqa	xmm5, xmm9
	psllq	xmm5, 2
	paddq	xmm5, xmm1
	test	bl, 64
	je	.LBB0_21
	movq	r14, xmm5
	movss	xmm10, dword ptr [r14]
	shufps	xmm10, xmm13, 48
	shufps	xmm13, xmm10, 132
.LBB0_21:
	test	bl, -128
	je	.LBB0_23
	pshufd	xmm5, xmm5, 238
	movq	rbx, xmm5
	movss	xmm5, dword ptr [rbx]
	shufps	xmm5, xmm13, 228
	shufps	xmm13, xmm5, 36
.LBB0_23:
	movdqa	xmm15, xmm9
	pcmpeqd	xmm0, xmm0
	paddq	xmm15, xmm0
	movdqa	xmm10, xmm4
	paddq	xmm10, xmm0
	movdqa	xmm11, xmm6
	paddq	xmm11, xmm0
	paddq	xmm8, xmm0
	movdqa	xmm14, xmm8
	psllq	xmm14, 2
	movdqa	xmm0, xmm7
	paddq	xmm14, xmm7
	movdqa	xmm7, xmmword ptr [rip + __xmm@80000000800000008000000080000000]
	pxor	xmm8, xmm7
	movdqa	xmm5, xmm2
	pcmpgtd	xmm5, xmm8
	pshufd	xmm1, xmm5, 160
	pcmpeqd	xmm8, xmm2
	pshufd	xmm8, xmm8, 245
	pand	xmm8, xmm1
	pshufd	xmm5, xmm5, 245
	por	xmm5, xmm8
	movdqa	xmm1, xmm11
	pxor	xmm1, xmm7
	movdqa	xmm8, xmm2
	pcmpgtd	xmm8, xmm1
	pshufd	xmm3, xmm8, 160
	pcmpeqd	xmm1, xmm2
	pshufd	xmm1, xmm1, 245
	pand	xmm1, xmm3
	pshufd	xmm3, xmm8, 245
	por	xmm3, xmm1
	packssdw	xmm5, xmm3
	movdqa	xmm1, xmm10
	pxor	xmm1, xmm7
	movdqa	xmm3, xmm2
	pcmpgtd	xmm3, xmm1
	pshufd	xmm8, xmm3, 160
	pcmpeqd	xmm1, xmm2
	pshufd	xmm1, xmm1, 245
	pand	xmm1, xmm8
	pshufd	xmm3, xmm3, 245
	por	xmm3, xmm1
	movdqa	xmm1, xmm15
	pxor	xmm1, xmm7
	movdqa	xmm8, xmm2
	pcmpgtd	xmm8, xmm1
	pshufd	xmm7, xmm8, 160
	pcmpeqd	xmm1, xmm2
	pshufd	xmm1, xmm1, 245
	pand	xmm1, xmm7
	pshufd	xmm7, xmm8, 245
	por	xmm7, xmm1
	packssdw	xmm3, xmm7
	packssdw	xmm5, xmm3
	packsswb	xmm5, xmm5
	pmovmskb	ebx, xmm5
	pxor	xmm5, xmm5
	pxor	xmm8, xmm8
	test	bl, 1
	je	.LBB0_25
	movq	r14, xmm14
	movd	xmm5, dword ptr [r14]
.LBB0_25:
	test	bl, 2
	je	.LBB0_27
	pshufd	xmm1, xmm14, 238
	movq	r14, xmm1
	movd	xmm1, dword ptr [r14]
	punpcklqdq	xmm1, xmm5
	shufps	xmm1, xmm5, 226
	movaps	xmm5, xmm1
.LBB0_27:
	movdqa	xmm7, xmm0
	psllq	xmm11, 2
	paddq	xmm11, xmm0
	test	bl, 4
	jne	.LBB0_28
	test	bl, 8
	jne	.LBB0_30
.LBB0_31:
	psllq	xmm10, 2
	paddq	xmm10, xmm7
	test	bl, 16
	jne	.LBB0_32
.LBB0_33:
	test	bl, 32
	jne	.LBB0_34
.LBB0_35:
	psllq	xmm15, 2
	paddq	xmm15, xmm7
	test	bl, 64
	jne	.LBB0_36
.LBB0_37:
	test	bl, -128
	je	.LBB0_39
.LBB0_38:
	pshufd	xmm1, xmm15, 238
	movq	rbx, xmm1
	movss	xmm1, dword ptr [rbx]
	shufps	xmm1, xmm8, 228
	shufps	xmm8, xmm1, 36
.LBB0_39:
	pshufd	xmm1, xmm5, 245
	pmuludq	xmm5, xmm12
	pshufd	xmm5, xmm5, 232
	pshufd	xmm3, xmm12, 245
	pmuludq	xmm3, xmm1
	pshufd	xmm1, xmm3, 232
	punpckldq	xmm5, xmm1
	paddd	xmm5, xmm5
	lea	rbx, [rdi - 2]
	cmp	rbx, rdx
	jb	.LBB0_58
	cmp	rdi, rdx
	jb	.LBB0_41
.LBB0_42:
	movq	rbx, xmm6
	add	rbx, rbx
	cmp	rbx, rdx
	jae	.LBB0_44
.LBB0_43:
	pshufd	xmm1, xmm5, 238
	movd	ebp, xmm1
	add	dword ptr [rsi + 4*rbx], ebp
.LBB0_44:
	pshufd	xmm1, xmm6, 238
	movq	rbx, xmm1
	add	rbx, rbx
	cmp	rbx, rdx
	jae	.LBB0_46
	pshufd	xmm1, xmm5, 255
	movd	ebp, xmm1
	add	dword ptr [rsi + 4*rbx], ebp
.LBB0_46:
	pshufd	xmm1, xmm8, 245
	pmuludq	xmm8, xmm13
	pshufd	xmm5, xmm8, 232
	pshufd	xmm3, xmm13, 245
	pmuludq	xmm3, xmm1
	pshufd	xmm1, xmm3, 232
	punpckldq	xmm5, xmm1
	paddd	xmm5, xmm5
	movq	rbx, xmm4
	add	rbx, rbx
	cmp	rbx, rdx
	jae	.LBB0_48
	movd	ebp, xmm5
	add	dword ptr [rsi + 4*rbx], ebp
.LBB0_48:
	pshufd	xmm1, xmm4, 238
	movq	rbx, xmm1
	add	rbx, rbx
	cmp	rbx, rdx
	jae	.LBB0_50
	pshufd	xmm1, xmm5, 85
	movd	ebp, xmm1
	add	dword ptr [rsi + 4*rbx], ebp
.LBB0_50:
	movq	rbx, xmm9
	add	rbx, rbx
	cmp	rbx, rdx
	jae	.LBB0_52
	pshufd	xmm1, xmm5, 238
	movd	ebp, xmm1
	add	dword ptr [rsi + 4*rbx], ebp
.LBB0_52:
	pshufd	xmm1, xmm9, 238
	movq	rbx, xmm1
	add	rbx, rbx
	cmp	rbx, rdx
	jae	.LBB0_54
	pshufd	xmm1, xmm5, 255
	movd	ebp, xmm1
	add	dword ptr [rsi + 4*rbx], ebp
	jmp	.LBB0_54
	.p2align	4
.LBB0_28:
	movq	r14, xmm11
	movss	xmm1, dword ptr [r14]
	shufps	xmm1, xmm5, 48
	shufps	xmm5, xmm1, 132
	test	bl, 8
	je	.LBB0_31
.LBB0_30:
	pshufd	xmm1, xmm11, 238
	movq	r14, xmm1
	movss	xmm1, dword ptr [r14]
	shufps	xmm1, xmm5, 228
	shufps	xmm5, xmm1, 36
	psllq	xmm10, 2
	paddq	xmm10, xmm7
	test	bl, 16
	je	.LBB0_33
.LBB0_32:
	movq	r14, xmm10
	movss	xmm1, dword ptr [r14]
	movss	xmm8, xmm1
	test	bl, 32
	je	.LBB0_35
.LBB0_34:
	pshufd	xmm1, xmm10, 238
	movq	r14, xmm1
	movss	xmm1, dword ptr [r14]
	movlhps	xmm1, xmm8
	shufps	xmm1, xmm8, 226
	movaps	xmm8, xmm1
	psllq	xmm15, 2
	paddq	xmm15, xmm7
	test	bl, 64
	je	.LBB0_37
.LBB0_36:
	movq	r14, xmm15
	movss	xmm1, dword ptr [r14]
	shufps	xmm1, xmm8, 48
	shufps	xmm8, xmm1, 132
	test	bl, -128
	jne	.LBB0_38
	jmp	.LBB0_39
	.p2align	4
.LBB0_58:
	movd	ebx, xmm5
	add	dword ptr [rsi + 8*rcx - 64], ebx
	cmp	rdi, rdx
	jae	.LBB0_42
.LBB0_41:
	pshufd	xmm1, xmm5, 85
	movd	ebx, xmm1
	add	dword ptr [rsi + 8*rcx - 56], ebx
	movq	rbx, xmm6
	add	rbx, rbx
	cmp	rbx, rdx
	jb	.LBB0_43
	jmp	.LBB0_44
.LBB0_59:
	lea	r8, [rip + anon.271030ac748f6b252f0b1933c629d6fc.2]
	call	_ZN4core9panicking18panic_bounds_check17h3ead85185f0acabaE
.LBB0_5:
	lea	r8, [rip + anon.271030ac748f6b252f0b1933c629d6fc.1]
	mov	rcx, r10
	mov	rdx, r9
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
	.asciz	"\025\000\000\000\000\000\000\0004\000\000\000\035\000\000"

	.section	.rdata,"dr",one_only,anon.271030ac748f6b252f0b1933c629d6fc.2
	.p2align	3, 0x0
anon.271030ac748f6b252f0b1933c629d6fc.2:
	.quad	anon.271030ac748f6b252f0b1933c629d6fc.0
	.asciz	"\025\000\000\000\000\000\000\0004\000\000\000\r\000\000"


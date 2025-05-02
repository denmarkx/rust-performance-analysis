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
	.globl	__xmm@0000000000000010000000000000000f
	.section	.rdata,"dr",discard,__xmm@0000000000000010000000000000000f
	.p2align	4, 0x0
__xmm@0000000000000010000000000000000f:
	.quad	15
	.quad	16
	.globl	__xmm@000000000000000e000000000000000d
	.section	.rdata,"dr",discard,__xmm@000000000000000e000000000000000d
	.p2align	4, 0x0
__xmm@000000000000000e000000000000000d:
	.quad	13
	.quad	14
	.globl	__xmm@000000000000000c000000000000000b
	.section	.rdata,"dr",discard,__xmm@000000000000000c000000000000000b
	.p2align	4, 0x0
__xmm@000000000000000c000000000000000b:
	.quad	11
	.quad	12
	.globl	__xmm@000000000000000a0000000000000009
	.section	.rdata,"dr",discard,__xmm@000000000000000a0000000000000009
	.p2align	4, 0x0
__xmm@000000000000000a0000000000000009:
	.quad	9
	.quad	10
	.globl	__xmm@00000000000000080000000000000007
	.section	.rdata,"dr",discard,__xmm@00000000000000080000000000000007
	.p2align	4, 0x0
__xmm@00000000000000080000000000000007:
	.quad	7
	.quad	8
	.globl	__xmm@00000000000000060000000000000005
	.section	.rdata,"dr",discard,__xmm@00000000000000060000000000000005
	.p2align	4, 0x0
__xmm@00000000000000060000000000000005:
	.quad	5
	.quad	6
	.globl	__xmm@00000000000000040000000000000003
	.section	.rdata,"dr",discard,__xmm@00000000000000040000000000000003
	.p2align	4, 0x0
__xmm@00000000000000040000000000000003:
	.quad	3
	.quad	4
	.globl	__xmm@00000000000000020000000000000001
	.section	.rdata,"dr",discard,__xmm@00000000000000020000000000000001
	.p2align	4, 0x0
__xmm@00000000000000020000000000000001:
	.quad	1
	.quad	2
	.globl	__xmm@00000000000000100000000000000010
	.section	.rdata,"dr",discard,__xmm@00000000000000100000000000000010
	.p2align	4, 0x0
__xmm@00000000000000100000000000000010:
	.quad	16
	.quad	16
	.section	.text,"xr",one_only,_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E
	.globl	_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E
	.p2align	4, 0x90
_ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E:
.seh_proc _ZN18auto_vectorization6update17h41dd6f2c05d0f5c0E
	sub	rsp, 88
	.seh_stackalloc 88
	movdqa	xmmword ptr [rsp + 64], xmm10
	.seh_savexmm xmm10, 64
	movdqa	xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa	xmmword ptr [rsp + 32], xmm8
	.seh_savexmm xmm8, 32
	movdqa	xmmword ptr [rsp + 16], xmm7
	.seh_savexmm xmm7, 16
	movdqa	xmmword ptr [rsp], xmm6
	.seh_savexmm xmm6, 0
	.seh_endprologue
	mov	r8, qword ptr [rcx + 16]
	xor	eax, eax
	cmp	r8, 2
	jb	.LBB0_10
	mov	eax, 1
	cmp	r8, 16
	jbe	.LBB0_2
	lea	r10, [r8 - 1]
	mov	r9, r10
	and	r9, -16
	lea	rax, [r9 + 1]
	movdqa	xmm0, xmmword ptr [rip + __xmm@0000000000000010000000000000000f]
	movdqa	xmm1, xmmword ptr [rip + __xmm@000000000000000e000000000000000d]
	movdqa	xmm3, xmmword ptr [rip + __xmm@000000000000000c000000000000000b]
	movdqa	xmm4, xmmword ptr [rip + __xmm@000000000000000a0000000000000009]
	movdqa	xmm5, xmmword ptr [rip + __xmm@00000000000000080000000000000007]
	movdqa	xmm6, xmmword ptr [rip + __xmm@00000000000000060000000000000005]
	movdqa	xmm7, xmmword ptr [rip + __xmm@00000000000000040000000000000003]
	movdqa	xmm8, xmmword ptr [rip + __xmm@00000000000000020000000000000001]
	pcmpeqd	xmm2, xmm2
	movdqa	xmm9, xmmword ptr [rip + __xmm@00000000000000100000000000000010]
	mov	r11, r9
	.p2align	4, 0x90
.LBB0_6:
	movdqa	xmm10, xmm0
	paddq	xmm8, xmm9
	paddq	xmm7, xmm9
	paddq	xmm6, xmm9
	paddq	xmm5, xmm9
	paddq	xmm4, xmm9
	paddq	xmm3, xmm9
	paddq	xmm1, xmm9
	paddq	xmm0, xmm9
	add	r11, -16
	jne	.LBB0_6
	cmp	r10, r9
	jne	.LBB0_2
	paddq	xmm10, xmm2
	pshufd	xmm0, xmm10, 238
	movq	rax, xmm0
	jmp	.LBB0_9
.LBB0_2:
	dec	rax
	neg	r8
	mov	r9, rax
	.p2align	4, 0x90
.LBB0_3:
	lea	rax, [r8 + r9]
	inc	rax
	inc	r9
	cmp	rax, -1
	jne	.LBB0_3
	lea	rax, [r9 - 1]
.LBB0_9:
	mov	rcx, qword ptr [rcx + 8]
	mov	rdx, qword ptr [rdx + 8]
	mov	eax, dword ptr [rdx + 4*rax]
	imul	eax, dword ptr [rcx + 4*r9]
.LBB0_10:
	movaps	xmm6, xmmword ptr [rsp]
	movaps	xmm7, xmmword ptr [rsp + 16]
	movaps	xmm8, xmmword ptr [rsp + 32]
	movaps	xmm9, xmmword ptr [rsp + 48]
	movaps	xmm10, xmmword ptr [rsp + 64]
	add	rsp, 88
	ret
	.seh_endproc


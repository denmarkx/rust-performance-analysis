	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.intel_syntax noprefix
	.file	"out_of_bounds.fef7a6df023ef0b5-cgu.0"
	.def	_ZN3std2rt10lang_start17h96ffc5e1d970b8efE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17h96ffc5e1d970b8efE
	.globl	_ZN3std2rt10lang_start17h96ffc5e1d970b8efE
	.p2align	4, 0x90
_ZN3std2rt10lang_start17h96ffc5e1d970b8efE:
.seh_proc _ZN3std2rt10lang_start17h96ffc5e1d970b8efE
	sub	rsp, 56
	.seh_stackalloc 56
	.seh_endprologue
	mov	rax, r8
	mov	r8, rdx
	mov	qword ptr [rsp + 48], rcx
	mov	byte ptr [rsp + 32], r9b
	lea	rdx, [rip + __unnamed_1]
	lea	rcx, [rsp + 48]
	mov	r9, rax
	call	_ZN3std2rt19lang_start_internal17ha007b66109036868E
	nop
	add	rsp, 56
	ret
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h15b49d7f42bc6968E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h15b49d7f42bc6968E
	.p2align	4, 0x90
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h15b49d7f42bc6968E:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h15b49d7f42bc6968E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rcx, qword ptr [rcx]
	call	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9e1d1ba9971fbbf0E
	xor	eax, eax
	add	rsp, 40
	ret
	.seh_endproc

	.def	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9e1d1ba9971fbbf0E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9e1d1ba9971fbbf0E
	.p2align	4, 0x90
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9e1d1ba9971fbbf0E:
.seh_proc _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9e1d1ba9971fbbf0E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	call	rcx
	#APP
	#NO_APP
	nop
	add	rsp, 40
	ret
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf346adbfbe1b8df8E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf346adbfbe1b8df8E
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf346adbfbe1b8df8E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf346adbfbe1b8df8E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rcx, qword ptr [rcx]
	call	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9e1d1ba9971fbbf0E
	xor	eax, eax
	add	rsp, 40
	ret
	.seh_endproc

	.def	_ZN13out_of_bounds5check17h3233c9f0359399b0E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN13out_of_bounds5check17h3233c9f0359399b0E
	.p2align	4, 0x90
_ZN13out_of_bounds5check17h3233c9f0359399b0E:
	ret

	.def	_ZN13out_of_bounds4main17h3d8ae2c38721b59cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	.p2align	4, 0x90
_ZN13out_of_bounds4main17h3d8ae2c38721b59cE:
.seh_proc _ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	push	rsi
	.seh_pushreg rsi
	sub	rsp, 64
	.seh_stackalloc 64
	.seh_endprologue
	movzx	eax, byte ptr [rip + __rust_no_alloc_shim_is_unstable]
	mov	ecx, 36
	mov	edx, 4
	call	__rust_alloc
	test	rax, rax
	je	.LBB5_2
	mov	rsi, rax
	mov	qword ptr [rsp + 40], 9
	mov	qword ptr [rsp + 48], rax
	mov	qword ptr [rsp + 56], 9
	lea	rcx, [rsp + 40]
	call	_ZN13out_of_bounds5check17h3233c9f0359399b0E
	mov	edx, 36
	mov	r8d, 4
	mov	rcx, rsi
	add	rsp, 64
	pop	rsi
	jmp	__rust_dealloc
.LBB5_2:
	mov	ecx, 4
	mov	edx, 36
	call	_ZN5alloc5alloc18handle_alloc_error17hecdf245a78500debE
	int3
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main
	.globl	main
	.p2align	4, 0x90
main:
.seh_proc main
	sub	rsp, 56
	.seh_stackalloc 56
	.seh_endprologue
	mov	r9, rdx
	movsxd	r8, ecx
	lea	rax, [rip + _ZN13out_of_bounds4main17h3d8ae2c38721b59cE]
	mov	qword ptr [rsp + 48], rax
	mov	byte ptr [rsp + 32], 0
	lea	rdx, [rip + __unnamed_1]
	lea	rcx, [rsp + 48]
	call	_ZN3std2rt19lang_start_internal17ha007b66109036868E
	nop
	add	rsp, 56
	ret
	.seh_endproc

	.section	.rdata,"dr",one_only,__unnamed_1
	.p2align	3, 0x0
__unnamed_1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf346adbfbe1b8df8E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h15b49d7f42bc6968E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h15b49d7f42bc6968E


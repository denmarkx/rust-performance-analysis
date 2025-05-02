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
.seh_proc _ZN13out_of_bounds5check17h3233c9f0359399b0E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	cmp	qword ptr [rcx + 16], 0
	je	.LBB4_2
	add	rsp, 40
	ret
.LBB4_2:
	lea	r8, [rip + __unnamed_2]
	xor	ecx, ecx
	xor	edx, edx
	call	_ZN4core9panicking18panic_bounds_check17h883675bd61368af7E
	int3
	.seh_endproc

	.def	_ZN13out_of_bounds4main17h3d8ae2c38721b59cE;
	.scl	3;
	.type	32;
	.endef
	.globl	__xmm@00000003000000020000000100000000
	.section	.rdata,"dr",discard,__xmm@00000003000000020000000100000000
	.p2align	4, 0x0
__xmm@00000003000000020000000100000000:
	.long	0
	.long	1
	.long	2
	.long	3
	.globl	__xmm@00000007000000060000000500000004
	.section	.rdata,"dr",discard,__xmm@00000007000000060000000500000004
	.p2align	4, 0x0
__xmm@00000007000000060000000500000004:
	.long	4
	.long	5
	.long	6
	.long	7
	.section	.text,"xr",one_only,_ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	.p2align	4, 0x90
_ZN13out_of_bounds4main17h3d8ae2c38721b59cE:
.Lfunc_begin0:
.seh_proc _ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 80
	.seh_stackalloc 80
	lea	rbp, [rsp + 80]
	.seh_setframe rbp, 80
	.seh_endprologue
	mov	qword ptr [rbp - 8], -2
	movzx	eax, byte ptr [rip + __rust_no_alloc_shim_is_unstable]
	mov	ecx, 36
	mov	edx, 4
	call	__rust_alloc
	test	rax, rax
	je	.LBB5_3
	movaps	xmm0, xmmword ptr [rip + __xmm@00000003000000020000000100000000]
	movups	xmmword ptr [rax], xmm0
	movaps	xmm0, xmmword ptr [rip + __xmm@00000007000000060000000500000004]
	movups	xmmword ptr [rax + 16], xmm0
	mov	dword ptr [rax + 32], 8
	mov	qword ptr [rbp - 40], 9
	mov	qword ptr [rbp - 16], rax
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 9
.Ltmp0:
	lea	rcx, [rbp - 40]
	call	_ZN13out_of_bounds5check17h3233c9f0359399b0E
.Ltmp1:
	mov	edx, 36
	mov	r8d, 4
	mov	rcx, qword ptr [rbp - 16]
	add	rsp, 80
	pop	rbp
	jmp	__rust_dealloc
.LBB5_3:
	mov	ecx, 4
	mov	edx, 36
	call	_ZN5alloc5alloc18handle_alloc_error17hecdf245a78500debE
	int3
	.seh_handlerdata
	.long	($cppxdata$_ZN13out_of_bounds4main17h3d8ae2c38721b59cE)@IMGREL
	.section	.text,"xr",one_only,_ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	.seh_endproc
	.def	"?dtor$4@?0?_ZN13out_of_bounds4main17h3d8ae2c38721b59cE@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$4@?0?_ZN13out_of_bounds4main17h3d8ae2c38721b59cE@4HA":
.seh_proc "?dtor$4@?0?_ZN13out_of_bounds4main17h3d8ae2c38721b59cE@4HA"
.LBB5_4:
	mov	qword ptr [rsp + 16], rdx
	push	rbp
	.seh_pushreg rbp
	sub	rsp, 32
	.seh_stackalloc 32
	lea	rbp, [rdx + 80]
	.seh_endprologue
	mov	edx, 36
	mov	r8d, 4
	mov	rcx, qword ptr [rbp - 16]
	call	__rust_dealloc
	nop
	add	rsp, 32
	pop	rbp
	ret
.Lfunc_end0:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN13out_of_bounds4main17h3d8ae2c38721b59cE
	.p2align	2, 0x0
$cppxdata$_ZN13out_of_bounds4main17h3d8ae2c38721b59cE:
	.long	429065506
	.long	1
	.long	($stateUnwindMap$_ZN13out_of_bounds4main17h3d8ae2c38721b59cE)@IMGREL
	.long	0
	.long	0
	.long	3
	.long	($ip2state$_ZN13out_of_bounds4main17h3d8ae2c38721b59cE)@IMGREL
	.long	72
	.long	0
	.long	1
$stateUnwindMap$_ZN13out_of_bounds4main17h3d8ae2c38721b59cE:
	.long	-1
	.long	"?dtor$4@?0?_ZN13out_of_bounds4main17h3d8ae2c38721b59cE@4HA"@IMGREL
$ip2state$_ZN13out_of_bounds4main17h3d8ae2c38721b59cE:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN13out_of_bounds4main17h3d8ae2c38721b59cE

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

	.section	.rdata,"dr",one_only,__unnamed_3
__unnamed_3:
	.ascii	"out_of_bounds.rs"

	.section	.rdata,"dr",one_only,__unnamed_2
	.p2align	3, 0x0
__unnamed_2:
	.quad	__unnamed_3
	.asciz	"\020\000\000\000\000\000\000\000\r\000\000\000\023\000\000"


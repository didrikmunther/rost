	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.p2align	4, 0x90
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h84cf9bb49dc0b862E:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	callq	__ZN4core3ops8function6FnOnce9call_once17h64eb21cc9fbde6acE
Ltmp0:
	callq	__ZN4core4hint9black_box17hab79d3926b475756E
Ltmp1:
	jmp	LBB0_4
LBB0_2:
	jmp	LBB0_5
LBB0_3:
Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB0_2
LBB0_4:
	addq	$16, %rsp
	popq	%rbp
	retq
LBB0_5:
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table0:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__ZN3std2rt10lang_start17h25a8f8c9426b0722E
	.globl	__ZN3std2rt10lang_start17h25a8f8c9426b0722E
	.p2align	4, 0x90
__ZN3std2rt10lang_start17h25a8f8c9426b0722E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, -8(%rbp)
	leaq	-8(%rbp), %rdi
	leaq	l___unnamed_1(%rip), %rsi
	callq	__ZN3std2rt19lang_start_internal17h3fd5cff071397f19E
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6755556d2528380bE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	(%rdi), %rdi
	callq	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h84cf9bb49dc0b862E
	callq	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h9e4949b754a5dc03E
	movb	%al, -1(%rbp)
	movb	-1(%rbp), %al
	movzbl	%al, %edi
	callq	__ZN3std7process8ExitCode6to_i3217h49edce36bb77b587E
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %eax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h0579e98f9191ae08E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movzbl	(%rdi), %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std7process8ExitCode6to_i3217h49edce36bb77b587E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movb	%dil, %al
	movb	%al, -1(%rbp)
	leaq	-1(%rbp), %rdi
	callq	__ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h0579e98f9191ae08E
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %eax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN47_$LT$i32$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17h4508f0267f6bbd7cE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
	movl	%esi, %eax
	addl	%eax, %edi
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %eax
	addq	$8, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3cmp5impls55_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$i32$GT$2lt17hf1778733993d8fc9E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	(%rdi), %eax
	cmpl	(%rsi), %eax
	setl	%al
	andb	$1, %al
	movzbl	%al, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt10ArgumentV111new_display17h679bdb5c1c5a9e3bE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h9eb052463410db30E@GOTPCREL(%rip), %rsi
	callq	__ZN4core3fmt10ArgumentV13new17h44599b831922b568E
	movq	%rax, -16(%rbp)
	movq	%rdx, -8(%rbp)
	movq	-8(%rbp), %rdx
	movq	-16(%rbp), %rax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt10ArgumentV13new17h44599b831922b568E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$56, %rsp
	movq	%rdi, -48(%rbp)
	movq	%rsi, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	-48(%rbp), %rax
	movq	%rax, -8(%rbp)
	movq	-8(%rbp), %rax
	movq	%rax, -56(%rbp)
	movq	-40(%rbp), %rax
	movq	-56(%rbp), %rcx
	movq	%rcx, -32(%rbp)
	movq	%rax, -24(%rbp)
	movq	-32(%rbp), %rax
	movq	-24(%rbp), %rdx
	addq	$56, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt9Arguments6new_v117hf91c7a6d49cc8b9fE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$128, %rsp
	movq	%r8, -120(%rbp)
	movq	%rcx, -112(%rbp)
	movq	%rdx, -104(%rbp)
	movq	%rsi, -96(%rbp)
	movq	%rdi, -88(%rbp)
	movq	%rdi, -80(%rbp)
	cmpq	%r8, %rdx
	jb	LBB9_2
	movq	-104(%rbp), %rax
	movq	-120(%rbp), %rcx
	addq	$1, %rcx
	cmpq	%rcx, %rax
	seta	%al
	andb	$1, %al
	movb	%al, -65(%rbp)
	jmp	LBB9_3
LBB9_2:
	movb	$1, -65(%rbp)
LBB9_3:
	testb	$1, -65(%rbp)
	jne	LBB9_5
	movq	-80(%rbp), %rax
	movq	-88(%rbp), %rcx
	movq	-120(%rbp), %rdx
	movq	-112(%rbp), %rsi
	movq	-104(%rbp), %rdi
	movq	-96(%rbp), %r8
	movq	$0, -16(%rbp)
	movq	%r8, (%rcx)
	movq	%rdi, 8(%rcx)
	movq	-16(%rbp), %r8
	movq	-8(%rbp), %rdi
	movq	%r8, 16(%rcx)
	movq	%rdi, 24(%rcx)
	movq	%rsi, 32(%rcx)
	movq	%rdx, 40(%rcx)
	addq	$128, %rsp
	popq	%rbp
	retq
LBB9_5:
	leaq	-64(%rbp), %rdi
	leaq	l___unnamed_2(%rip), %rsi
	movl	$1, %edx
	leaq	l___unnamed_3(%rip), %rcx
	xorl	%eax, %eax
	movl	%eax, %r8d
	callq	__ZN4core3fmt9Arguments6new_v117hf91c7a6d49cc8b9fE
	leaq	l___unnamed_4(%rip), %rsi
	leaq	-64(%rbp), %rdi
	callq	__ZN4core9panicking9panic_fmt17h3d9f795ee387ef8dE
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3mem7replace17hba4e028e4f608e48E:
Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception1
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -40(%rbp)
	movl	%esi, -28(%rbp)
	movb	$1, -17(%rbp)
Ltmp3:
	callq	__ZN4core3ptr4read17heeca38ead0998187E
Ltmp4:
	movl	%eax, -24(%rbp)
	jmp	LBB10_3
LBB10_1:
	testb	$1, -17(%rbp)
	jne	LBB10_8
	jmp	LBB10_7
LBB10_2:
Ltmp5:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB10_1
LBB10_3:
	movl	-28(%rbp), %esi
	movq	-40(%rbp), %rdi
	movb	$0, -17(%rbp)
Ltmp6:
	callq	__ZN4core3ptr5write17hc66eb8f7150248deE
Ltmp7:
	jmp	LBB10_6
LBB10_4:
	jmp	LBB10_1
LBB10_5:
Ltmp8:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB10_4
LBB10_6:
	movl	-24(%rbp), %eax
	addq	$48, %rsp
	popq	%rbp
	retq
LBB10_7:
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
LBB10_8:
	jmp	LBB10_7
Lfunc_end1:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table10:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp3-Lfunc_begin1
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp6-Lfunc_begin1
	.uleb128 Ltmp7-Ltmp6
	.uleb128 Ltmp8-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp7-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp7
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4d11038fe2e20a63E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	(%rdi), %rdi
	callq	__ZN4core3ops8function6FnOnce9call_once17hdb25cb9be3e5c05eE
	movl	%eax, -12(%rbp)
	movl	-12(%rbp), %eax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17h64eb21cc9fbde6acE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	callq	*%rdi
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17hdb25cb9be3e5c05eE:
Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception2
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -32(%rbp)
Ltmp9:
	leaq	-32(%rbp), %rdi
	callq	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6755556d2528380bE
Ltmp10:
	movl	%eax, -36(%rbp)
	jmp	LBB13_3
LBB13_1:
	jmp	LBB13_4
LBB13_2:
Ltmp11:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB13_1
LBB13_3:
	jmp	LBB13_5
LBB13_4:
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
LBB13_5:
	movl	-36(%rbp), %eax
	addq	$48, %rsp
	popq	%rbp
	retq
Lfunc_end2:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table13:
Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp9-Lfunc_begin2
	.uleb128 Ltmp10-Ltmp9
	.uleb128 Ltmp11-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp10-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp10
	.byte	0
	.byte	0
Lcst_end2:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN4core3ptr4read17heeca38ead0998187E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$24, %rsp
	movq	%rdi, -16(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, -8(%rbp)
	jmp	LBB14_2
LBB14_2:
	movq	-16(%rbp), %rax
	movl	(%rax), %eax
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %eax
	movl	%eax, -20(%rbp)
	movl	-20(%rbp), %eax
	addq	$24, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ptr5write17hc66eb8f7150248deE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$4, %rsp
	movl	%esi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, (%rdi)
	addq	$4, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7c17c7bd93956393E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core4hint9black_box17hab79d3926b475756E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	## InlineAsm Start
	## InlineAsm End
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h07d1154b87304093E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	callq	__ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17h5470a6613541860fE
	movl	%eax, -8(%rbp)
	movl	%edx, -4(%rbp)
	movl	-4(%rbp), %edx
	movl	-8(%rbp), %eax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core5clone5impls52_$LT$impl$u20$core..clone..Clone$u20$for$u20$i32$GT$5clone17hbb41de5b319d52c6E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	(%rdi), %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h9e4949b754a5dc03E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hcfbfdbdfed6c6e79E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	%esi, %edx
	movl	%edi, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17h5470a6613541860fE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, %rsi
	movq	%rsi, -24(%rbp)
	movq	%rsi, %rdi
	addq	$4, %rsi
	callq	__ZN4core3cmp5impls55_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$i32$GT$2lt17hf1778733993d8fc9E
	movb	%al, -9(%rbp)
	movb	-9(%rbp), %al
	testb	$1, %al
	jne	LBB22_3
	jmp	LBB22_2
LBB22_2:
	movl	$0, -8(%rbp)
	jmp	LBB22_7
LBB22_3:
	movq	-24(%rbp), %rdi
	callq	__ZN4core5clone5impls52_$LT$impl$u20$core..clone..Clone$u20$for$u20$i32$GT$5clone17hbb41de5b319d52c6E
	movl	%eax, -28(%rbp)
	movl	-28(%rbp), %edi
	movl	$1, %esi
	callq	__ZN47_$LT$i32$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17h4508f0267f6bbd7cE
	movl	%eax, -32(%rbp)
	movl	-32(%rbp), %esi
	movq	-24(%rbp), %rdi
	callq	__ZN4core3mem7replace17hba4e028e4f608e48E
	movl	%eax, -36(%rbp)
	movl	-36(%rbp), %eax
	movl	%eax, -4(%rbp)
	movl	$1, -8(%rbp)
LBB22_7:
	movl	-8(%rbp), %eax
	movl	-4(%rbp), %edx
	addq	$48, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4test4main17h4a9b7191fa98c2a6E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$128, %rsp
	movl	$0, -96(%rbp)
	movl	$1000000, -92(%rbp)
	movl	-96(%rbp), %edi
	movl	-92(%rbp), %esi
	callq	__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hcfbfdbdfed6c6e79E
	movl	%eax, -104(%rbp)
	movl	%edx, -100(%rbp)
	movl	-100(%rbp), %eax
	movl	-104(%rbp), %ecx
	movl	%ecx, -88(%rbp)
	movl	%eax, -84(%rbp)
LBB23_2:
	leaq	-88(%rbp), %rdi
	callq	__ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h07d1154b87304093E
	movl	%edx, -76(%rbp)
	movl	%eax, -80(%rbp)
	movl	-80(%rbp), %eax
	testq	%rax, %rax
	je	LBB23_5
	jmp	LBB23_9
LBB23_9:
	jmp	LBB23_6
	ud2
LBB23_5:
	addq	$128, %rsp
	popq	%rbp
	retq
LBB23_6:
	movl	-76(%rbp), %eax
	movl	%eax, -68(%rbp)
	leaq	-68(%rbp), %rdi
	callq	__ZN4core3fmt10ArgumentV111new_display17h679bdb5c1c5a9e3bE
	movq	%rax, -120(%rbp)
	movq	%rdx, -112(%rbp)
	movq	-112(%rbp), %rax
	movq	-120(%rbp), %rcx
	movq	%rcx, -16(%rbp)
	movq	%rax, -8(%rbp)
	leaq	-16(%rbp), %rcx
	leaq	-64(%rbp), %rdi
	leaq	l___unnamed_5(%rip), %rsi
	movl	$2, %edx
	movl	$1, %r8d
	callq	__ZN4core3fmt9Arguments6new_v117hf91c7a6d49cc8b9fE
	leaq	-64(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17h3f48a1f06c5dc1b5E
	jmp	LBB23_2
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	__ZN4test4main17h4a9b7191fa98c2a6E(%rip), %rdi
	callq	__ZN3std2rt10lang_start17h25a8f8c9426b0722E
	popq	%rbp
	retq
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7c17c7bd93956393E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4d11038fe2e20a63E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6755556d2528380bE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6755556d2528380bE

	.section	__TEXT,__const
l___unnamed_6:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3
l___unnamed_2:
	.quad	l___unnamed_6
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3
l___unnamed_3:
	.byte	0

l___unnamed_7:
	.ascii	"/rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_4:
	.quad	l___unnamed_7
	.asciz	"K\000\000\000\000\000\000\000\207\001\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_8:
	.ascii	"/rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/ptr/mod.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_9:
	.quad	l___unnamed_8
	.asciz	"K\000\000\000\000\000\000\000\200\004\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_10:
	.byte	10

	.section	__DATA,__const
	.p2align	3
l___unnamed_5:
	.quad	l___unnamed_3
	.space	8
	.quad	l___unnamed_10
	.asciz	"\001\000\000\000\000\000\000"

.subsections_via_symbols

; ModuleID = 'probe6.24002e03-cgu.0'
source_filename = "probe6.24002e03-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_cb067141a2953e505cd83c4379abd118 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/da7c50c089d5db2d3ebaf227fe075bb1346bfaec/library/core/src/num/mod.rs" }>, align 1
@alloc_955fb7a67f93c245f1e5fd74a5edc77b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_cb067141a2953e505cd83c4379abd118, [16 x i8] c"K\00\00\00\00\00\00\00/\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: uwtable
define void @_ZN6probe65probe17h8820a3ce5f0b9860E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h326a959102fe5e0dE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc8eadf36d168ba5eE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_955fb7a67f93c245f1e5fd74a5edc77b) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h326a959102fe5e0dE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hc8eadf36d168ba5eE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}

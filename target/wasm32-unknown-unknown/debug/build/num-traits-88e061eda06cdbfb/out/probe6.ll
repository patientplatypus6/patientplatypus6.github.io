; ModuleID = 'probe6.24002e03-cgu.0'
source_filename = "probe6.24002e03-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_cb067141a2953e505cd83c4379abd118 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/da7c50c089d5db2d3ebaf227fe075bb1346bfaec/library/core/src/num/mod.rs" }>, align 1
@alloc_382209365d7a49abc023b40b4d44f866 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_cb067141a2953e505cd83c4379abd118, [12 x i8] c"K\00\00\00/\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe65probe17h8820a3ce5f0b9860E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hcae31c307dfba1aaE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb5fb2acb822dc391E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_382209365d7a49abc023b40b4d44f866) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hcae31c307dfba1aaE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17hb5fb2acb822dc391E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #3 = { noreturn nounwind }

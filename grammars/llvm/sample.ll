; A sample.
target triple = "x86_64-unknown-linux-gnu"

@limit = constant i32 10
@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00"

declare i32 @printf(i8*, ...)

define i32 @largest(i32 %a, i32 %b) {
entry:
  %cmp = icmp sgt i32 %a, %b
  br i1 %cmp, label %first, label %second

first:
  ret i32 %a

second:
  ret i32 %b
}

define i32 @main() {
entry:
  %limit = load i32, i32* @limit
  %best = call i32 @largest(i32 3, i32 %limit)
  %sum = add nsw i32 %best, 1
  %fmt = getelementptr inbounds [4 x i8], [4 x i8]* @.str, i64 0, i64 0
  %ignored = call i32 (i8*, ...) @printf(i8* %fmt, i32 %sum)
  ret i32 0
}

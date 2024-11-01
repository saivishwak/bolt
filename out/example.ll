; ModuleID = 'example_module'
source_filename = "example_module"

@msg = private unnamed_addr constant [11 x i8] c"Value: 10\0A\00", align 1

define i64 @main() {
entry:
  %b = alloca float, align 4
  %printf_call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([11 x i8], [11 x i8]* @msg, i32 0, i32 0))
  ret i64 10
}

declare i32 @printf(i8*, ...)

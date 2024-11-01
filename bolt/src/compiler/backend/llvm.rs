use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::{env, fs, io, ptr};

use crate::parser::ast::{
    BinaryExpression, BlockStatement, Boolean, Expression, ExpressionStatement, Identifier,
    IfExpression, IntegerLiteral, LetStatement, Program,
};
use crate::{compiler::Compiler, parser::ast::Statement};
use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::{core::*, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

fn cstring_from_string(str: String) -> CString {
    CString::new(str).expect("Error Converting to cstring")
}

struct MapValue {
    value_pointer: *mut LLVMValue,
    ident_pointer: *mut LLVMValue,
    ident_type: *mut LLVMType,
}

impl MapValue {
    pub(crate) fn new(
        value_pointer: *mut LLVMValue,
        ident_pointer: *mut LLVMValue,
        ident_type: *mut LLVMType,
    ) -> Self {
        Self {
            value_pointer,
            ident_pointer,
            ident_type,
        }
    }
}

pub struct LLVM {
    program: Rc<Program>,
    filename: String,
    context: Option<*mut LLVMContext>,
    module: Option<*mut LLVMModule>,
    builder: Option<*mut LLVMBuilder>,
    allocs: HashMap<CString, MapValue>,
    current_function: Option<LLVMValueRef>,
    print_function: Option<LLVMValueRef>,
    print_function_ty: Option<*mut LLVMType>,
}

impl LLVM {
    pub fn new(program: Program, filename: &str) -> Self {
        let mut instance = Self {
            program: Rc::new(program),
            context: None,
            module: None,
            builder: None,
            allocs: HashMap::new(),
            filename: filename.into(),
            current_function: None,
            print_function: None,
            print_function_ty: None,
        };
        unsafe {
            instance.set_environment();
        }
        instance
    }

    fn builder(&mut self) -> *mut LLVMBuilder {
        self.builder.unwrap()
    }

    fn context(&mut self) -> *mut LLVMContext {
        self.context.unwrap()
    }

    fn module(&mut self) -> *mut LLVMModule {
        self.module.unwrap()
    }

    pub unsafe fn set_environment(&mut self) {
        self.create_context();
        self.create_module();
        self.create_builder();
        let printf_type = LLVMFunctionType(
            LLVMInt32Type(),
            [LLVMPointerType(LLVMInt8Type(), 0), LLVMDoubleType()].as_mut_ptr(),
            2,
            1,
        );
        let printf_fn =
            LLVMAddFunction(self.module(), b"printf\0".as_ptr() as *const _, printf_type);
        self.print_function = Some(printf_fn);
        self.print_function_ty = Some(printf_type)
    }

    unsafe fn create_context(&mut self) {
        let context = LLVMContextCreate();
        self.context = Some(context);
    }

    unsafe fn create_module(&mut self) {
        let module_name = format!("{}_module\0", self.filename);
        let module = LLVMModuleCreateWithName(module_name.as_ptr() as *const _);
        self.module = Some(module);
    }

    unsafe fn create_builder(&mut self) {
        if let Some(context) = self.context {
            let builder = LLVMCreateBuilderInContext(context);
            self.builder = Some(builder);
        } else {
            panic!("Context not created!")
        }
    }

    unsafe fn set_main_func(&mut self) {
        let context = self.context();
        let module = self.module();
        let builder = self.builder();
        let function_type = LLVMFunctionType(LLVMVoidType(), ptr::null_mut(), 0, 0);
        let function = LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);
        let entry_name = CString::new("entry").unwrap();
        let bb = LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
        self.current_function = Some(function);
        LLVMPositionBuilderAtEnd(builder, bb);
    }

    unsafe fn set_return_main_func(&mut self) {
        let builder = self.builder();
        unsafe {
            LLVMBuildRetVoid(builder);
        }
    }

    unsafe fn compile_statement(&mut self, statement: &Box<dyn Statement>) {
        let value_any = statement.as_any();
        if let Some(stmt) = value_any.downcast_ref::<LetStatement>() {
            self.compile_let_statement(stmt);
        } else if let Some(expr) = value_any.downcast_ref::<ExpressionStatement>() {
            self.compile_expression(&expr.value);
        } else {
            panic!("Error compiling statment")
        }
    }

    unsafe fn compile_let_statement(&mut self, statement: &LetStatement) {
        let ident = &statement.identifier;
        match self.compile_expression(&statement.value) {
            Some(val) => {
                let (reference, _, obj_type, skip_alloc) = val;
                if !skip_alloc {
                    self.alloc(
                        CString::new(ident.value.clone()).unwrap(),
                        reference,
                        obj_type,
                    );
                } else {
                    let load = LLVMBuildLoad2(
                        self.builder(),
                        obj_type,
                        reference,
                        cstring_from_string(ident.value.clone()).as_ptr(),
                    );

                    self.add_print_function(load);
                }
            }
            None => {}
        }
    }

    unsafe fn compile_binary_expression(
        &mut self,
        left: Option<(LLVMValueRef, Option<LLVMValueRef>, *mut LLVMType, bool)>,
        right: Option<(LLVMValueRef, Option<LLVMValueRef>, *mut LLVMType, bool)>,
        operator: String,
    ) -> (LLVMValueRef, Option<LLVMValueRef>, *mut LLVMType, bool) {
        let builder = self.builder();
        let left = left.unwrap();
        let right = right.unwrap();
        //TODO Check TYPE
        match operator.as_str() {
            "+" => {
                // Assuming left and right are of the same type
                let left_value: *mut LLVMValue;
                let right_value: *mut LLVMValue;
                match left.1 {
                    Some(val) => {
                        left_value = val;
                    }
                    None => {
                        left_value = left.0;
                    }
                }
                match right.1 {
                    Some(val) => {
                        right_value = val;
                    }
                    None => {
                        right_value = right.0;
                    }
                }
                // Perform the addition
                let sum = LLVMBuildFAdd(builder, left_value, right_value, c_str!("sum"));

                //Return same type as LHS
                (sum, None, left.2, false)
            }
            "*" => {
                // Assuming left and right are of the same type

                let left_value: *mut LLVMValue;
                let right_value: *mut LLVMValue;
                match left.1 {
                    Some(val) => {
                        left_value = val;
                    }
                    None => {
                        left_value = left.0;
                    }
                }
                match right.1 {
                    Some(val) => {
                        right_value = val;
                    }
                    None => {
                        right_value = right.0;
                    }
                }

                // Perform the addition
                let sum = LLVMBuildFMul(builder, left_value, right_value, c_str!("mul"));

                //Return same type as LHS
                (sum, None, left.2, false)
            }
            _ => {
                panic!("Error")
            }
        }
    }

    unsafe fn compile_identifier(&mut self, identifier: &Identifier) -> &MapValue {
        let ident_string = cstring_from_string(identifier.value.clone());
        match self.allocs.get(&ident_string) {
            Some(reference) => {
                return reference;
            }
            None => panic!("Variable not found in store, Might not be initialized!"),
        }
    }

    unsafe fn compile_block(&mut self, block: &Box<BlockStatement>) {
        for stmt in &block.statements {
            unsafe { self.compile_statement(stmt) };
        }
    }

    unsafe fn compile_if_expression(&mut self, if_expression: &IfExpression) {
        let builder = self.builder();
        match self.compile_expression(&if_expression.condition) {
            Some(condition) => {
                let if_block =
                    LLVMAppendBasicBlock(self.current_function.unwrap(), c_str!("if_block"));
                let else_block =
                    LLVMAppendBasicBlock(self.current_function.unwrap(), c_str!("else_block"));

                // let casted_condition = LLVMBuildFCmp(
                //     builder,
                //     llvm_sys::LLVMRealPredicate::LLVMRealONE,
                //     condition.0,
                //     LLVMConstReal(LLVMDoubleType(), 0.0),
                //     c_str!("float_cmp"),
                // );
                // LLVMBuildZExt(
                //     builder,
                //     casted_condition,
                //     LLVMInt1Type(),
                //     c_str!("cast_to_i1"),
                // );
                let loaded_condition = LLVMBuildLoad2(
                    builder,
                    LLVMInt1Type(),
                    condition.0,
                    c_str!("loaded_condition"),
                ); //Only reference is allowed
                LLVMBuildCondBr(builder, loaded_condition, if_block, else_block);
                LLVMPositionBuilderAtEnd(builder, if_block);
                self.compile_block(&if_expression.consequence);
                LLVMBuildRetVoid(builder); //as we are in main function we need to exit the main function in if

                LLVMPositionBuilderAtEnd(builder, else_block);
                if let Some(else_branch) = &if_expression.alternate {
                    self.compile_block(else_branch);
                }
            }
            None => {}
        }
    }

    unsafe fn compile_expression(
        &mut self,
        expr: &Box<dyn Expression>,
    ) -> Option<(LLVMValueRef, Option<LLVMValueRef>, *mut LLVMType, bool)> {
        let value_any = expr.as_any();
        if let Some(int) = value_any.downcast_ref::<IntegerLiteral>() {
            let f64_type = LLVMDoubleTypeInContext(self.context());
            let reference = LLVMConstReal(f64_type, int.value);
            return Some((reference, None, f64_type, false));
        } else if let Some(binary) = value_any.downcast_ref::<BinaryExpression>() {
            let left = self.compile_expression(&binary.left);
            let right = self.compile_expression(&binary.right);
            return Some(self.compile_binary_expression(left, right, binary.operator.clone()));
        } else if let Some(boolean) = value_any.downcast_ref::<Boolean>() {
            if boolean.value == true {
                let true_value = LLVMConstInt(LLVMInt1Type(), 1, 0); // Represents `true`
                return Some((true_value, None, LLVMInt1Type(), false));
            } else {
                let false_value = LLVMConstInt(LLVMInt1Type(), 0, 0);
                return Some((false_value, None, LLVMInt1Type(), false));
            }
        } else if let Some(ident) = value_any.downcast_ref::<Identifier>() {
            let map_val = self.compile_identifier(ident);
            return Some((
                map_val.ident_pointer,
                Some(map_val.value_pointer),
                map_val.ident_type,
                true,
            ));
        } else if let Some(if_expression) = value_any.downcast_ref::<IfExpression>() {
            self.compile_if_expression(if_expression);
            return None;
        } else {
            panic!("Error compiling epxression")
        }
    }

    unsafe fn alloc(&mut self, ident: CString, pointer: *mut LLVMValue, obj_type: *mut LLVMType) {
        let builder = self.builder();
        let value_index_ptr = LLVMBuildAlloca(builder, obj_type, ident.as_ptr());
        LLVMBuildStore(builder, pointer, value_index_ptr);
        self.allocs
            .insert(ident, MapValue::new(pointer, value_index_ptr, obj_type));
    }

    unsafe fn add_print_function(&mut self, val: *mut LLVMValue) {
        let builder = self.builder();
        // Create the printf function type

        let msg = CString::new("Value: %f\n").unwrap();
        let msg_ptr = LLVMBuildGlobalStringPtr(
            builder,
            msg.as_ptr() as *const _,
            b"msg\0".as_ptr() as *const _,
        );

        // Call printf using LLVMBuildCall2
        LLVMBuildCall2(
            builder,
            self.print_function_ty.unwrap(),
            self.print_function.unwrap(),
            [msg_ptr, val].as_mut_ptr(),
            2,
            b"printf_call\0".as_ptr() as *const _,
        );
    }

    unsafe fn to_string(&mut self) -> String {
        let raw_str = LLVMPrintModuleToString(self.module());
        let c_str = unsafe { CStr::from_ptr(raw_str) };
        let str_value = c_str.to_string_lossy().into_owned();
        str_value
    }

    unsafe fn dispose(&mut self) {
        LLVMDisposeBuilder(self.builder());
        LLVMDisposeModule(self.module());
        LLVMContextDispose(self.context());
    }
}

impl Compiler for LLVM {
    fn compile(&mut self) {
        let program = self.program.clone();
        unsafe { self.set_main_func() };
        for stmt in &program.stmts {
            unsafe { self.compile_statement(stmt) };
        }
        unsafe { self.set_return_main_func() };
    }

    fn generate_ir(&mut self) -> String {
        unsafe { self.to_string() }
    }

    fn ir_to_file(&mut self, filename: String) {
        let out_file = CString::new(filename.as_str()).unwrap();
        unsafe { LLVMPrintModuleToFile(self.module(), out_file.as_ptr(), ptr::null_mut()) };
    }

    fn bytecode_to_file(&mut self, filename: String, target: &String) {
        let out_file = CString::new(filename.as_str()).unwrap();
        let name = CString::new(target.as_str()).expect("Error");
        unsafe {
            LLVMSetTarget(self.module(), name.as_ptr());
            LLVMWriteBitcodeToFile(self.module(), out_file.as_ptr());
        }
    }

    //TODO: Fix this bad JIT
    fn bytecode_to_jit(&mut self, _target: &String) {
        // Step 1: Generate assembly from LLVM IR
        let output_dir = setup_output_directory().unwrap();
        self.compile();

        // Construct the absolute path for the output file
        let output_path = output_dir
            .join("example.ll")
            .as_os_str()
            .to_string_lossy()
            .to_string();

        self.ir_to_file(output_path.clone());

        let assembly_file = output_dir
            .join("./example.s")
            .as_os_str()
            .to_string_lossy()
            .to_string();
        let status = Command::new("llc")
            .args(&["-relocation-model=pic", &output_path, "-o", &assembly_file])
            .status()
            .unwrap();

        if !status.success() {
            eprintln!("Failed to generate assembly.");
        }

        // Step 2: Assemble the assembly code into an object file
        let object_file = output_dir
            .join("./example.o")
            .as_os_str()
            .to_string_lossy()
            .to_string();
        let status = Command::new("as")
            .args(&[&assembly_file, "-o", &object_file])
            .status()
            .unwrap();

        if !status.success() {
            eprintln!("Failed to assemble.");
        }

        // Step 3: Link the object file and create an executable
        let executable_file = output_dir
            .join("./example")
            .as_os_str()
            .to_string_lossy()
            .to_string();
        let status = Command::new("gcc")
            .args(&[&object_file, "-o", &executable_file, "-lc"])
            .status()
            .unwrap();

        if !status.success() {
            eprintln!("Failed to link.");
        }

        // Step 4: Execute the resulting binary
        let status = Command::new(&executable_file).output().unwrap();

        print_output("example", &status);
    }

    fn clean(&mut self) {
        unsafe { self.dispose() };
    }
}

fn print_output(command: &str, output: &std::process::Output) {
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output from {}:\n{}", command, stdout);
    }
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error from {}:\n{}", command, stderr);
    }
}

fn setup_output_directory() -> io::Result<PathBuf> {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Construct the absolute path for the output file
    let output_path = current_dir.join("out");
    let output_dir = PathBuf::from(output_path);

    // Check if the output directory exists
    if output_dir.exists() {
        // If it exists, remove all files in the directory
        for entry in fs::read_dir(&output_dir)? {
            let entry = entry?;
            fs::remove_file(entry.path())?; // Remove the file
        }
    } else {
        // If it doesn't exist, create it
        fs::create_dir(&output_dir)?;
    }

    Ok(output_dir)
}

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;
use std::rc::Rc;
use std::thread::panicking;

use crate::evaluator::constants::{FALSE, TRUE};
use crate::object::object::{Interger, Object};
use crate::parser::ast::{Boolean, Expression, Identifier, IntegerLiteral, LetStatement, Program};
use crate::{compiler::Compiler, parser::ast::Statement};
use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::{core::*, LLVMBuilder, LLVMContext, LLVMModule, LLVMValue};

pub struct LLVM {
    program: Rc<Program>,
    context: Option<*mut LLVMContext>,
    module: Option<*mut LLVMModule>,
    builder: Option<*mut LLVMBuilder>,
    allocs: HashMap<CString, *mut LLVMValue>,
}

impl LLVM {
    pub unsafe fn new(program: Program) -> Self {
        let mut instance = Self {
            program: Rc::new(program),
            context: None,
            module: None,
            builder: None,
            allocs: HashMap::new(),
        };
        instance.create_context();
        instance.create_module();
        instance.create_builder();
        instance.set_main_func();
        instance
    }

    unsafe fn create_context(&mut self) -> &mut Self {
        let context = LLVMContextCreate();
        self.context = Some(context);
        self
    }

    unsafe fn create_module(&mut self) -> &mut Self {
        let module = LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
        self.module = Some(module);
        self
    }

    unsafe fn create_builder(&mut self) -> &mut Self {
        if let Some(context) = self.context {
            let builder = LLVMCreateBuilderInContext(context);
            self.builder = Some(builder);
        } else {
            panic!("Context not created!")
        }
        self
    }

    unsafe fn set_main_func(&mut self) -> &mut Self {
        let int_type = LLVMInt64TypeInContext(self.context.unwrap());
        let function_type = LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
        let function = LLVMAddFunction(
            self.module.unwrap(),
            b"main\0".as_ptr() as *const _,
            function_type,
        );
        let entry_name = CString::new("entry").unwrap();
        let bb =
            LLVMAppendBasicBlockInContext(self.context.unwrap(), function, entry_name.as_ptr());
        LLVMPositionBuilderAtEnd(self.builder.unwrap(), bb);
        self
    }

    unsafe fn alloc(&mut self, ident: &Identifier, obj: &Box<dyn Object>) {
        let context = self.context.unwrap();

        // Create the printf function type
        let printf_type = LLVMFunctionType(
            LLVMInt32Type(),
            [LLVMPointerType(LLVMInt8Type(), 0)].as_mut_ptr(),
            1,
            1,
        );
        let printf_fn = LLVMAddFunction(
            self.module.unwrap(),
            b"printf\0".as_ptr() as *const _,
            printf_type,
        );
        let obj_as_any = obj.as_any();
        if let Some(val) = obj_as_any.downcast_ref::<Interger>() {
            let int_type = LLVMFloatTypeInContext(context);
            let name = CString::new(ident.token.literal.clone()).unwrap();
            let pointer = LLVMBuildAlloca(self.builder.unwrap(), int_type, name.as_ptr());
            self.allocs.insert(name.to_owned(), pointer);
            // Create the message to print
            let msg = CString::new(format!("Value: {}\n", val.value).as_str()).unwrap();
            let msg_ptr = LLVMBuildGlobalStringPtr(
                self.builder.unwrap(),
                msg.as_ptr() as *const _,
                b"msg\0".as_ptr() as *const _,
            );

            // Call printf using LLVMBuildCall2
            LLVMBuildCall2(
                self.builder.unwrap(),
                printf_type,
                printf_fn,
                [msg_ptr].as_mut_ptr(),
                1,
                b"printf_call\0".as_ptr() as *const _,
            );
        } else {
            panic!("Error downcasting object")
        }
    }

    unsafe fn compile_expression(
        &mut self,
        expr: &Box<dyn Expression>,
    ) -> (Rc<Box<dyn Object>>, LLVMValueRef) {
        let value_any = expr.as_any();
        if let Some(int) = value_any.downcast_ref::<IntegerLiteral>() {
            let int_type = LLVMInt64TypeInContext(self.context.unwrap());
            let reference = LLVMConstInt(int_type, int.value as u64, 0);
            return (Rc::new(Box::new(Interger { value: int.value })), reference);
        } else {
            panic!("Error compiling epxression")
        }
    }

    fn compile_let_statement(&mut self, statement: &LetStatement) {
        // In LLVM, you get your types from functions.
        let ident = &statement.identifier;
        let (object, reference) = unsafe { self.compile_expression(&statement.value) };
        unsafe {
            self.alloc(ident, object.as_ref());
            LLVMBuildRet(self.builder.unwrap(), reference);
        }
    }

    unsafe fn to_string(&mut self) -> String {
        let raw_str = LLVMPrintModuleToString(self.module.unwrap());
        let c_str = unsafe { CStr::from_ptr(raw_str) };
        let str_value = c_str.to_string_lossy().into_owned();
        str_value
    }

    pub unsafe fn dispose(&mut self) {
        LLVMDisposeBuilder(self.builder.unwrap());
        LLVMDisposeModule(self.module.unwrap());
        LLVMContextDispose(self.context.unwrap());
    }
}

impl Compiler for LLVM {
    fn compile(&mut self) -> &mut Self {
        let program = self.program.clone();
        for stmt in &program.stmts {
            self.compile_statement(stmt);
        }
        self
    }

    fn compile_statement(&mut self, statement: &Box<dyn Statement>) {
        let value_any = statement.as_any();
        if let Some(stmt) = value_any.downcast_ref::<LetStatement>() {
            self.compile_let_statement(stmt);
        } else {
            panic!("Error compiling statment")
        }
    }

    fn generate_ir(&mut self) -> String {
        unsafe { self.to_string() }
    }

    fn ir_to_file(&mut self, filename: String) {
        let out_file = CString::new(filename.as_str()).unwrap();
        unsafe { LLVMPrintModuleToFile(self.module.unwrap(), out_file.as_ptr(), ptr::null_mut()) };
    }

    fn bytecode_to_file(&mut self, filename: String, target: &String) {
        let out_file = CString::new(filename.as_str()).unwrap();
        unsafe {
            LLVMSetTarget(
                self.module.unwrap(),
                CString::new(target.as_str()).unwrap().as_ptr(),
            );
            LLVMWriteBitcodeToFile(self.module.unwrap(), out_file.as_ptr());
        }
    }

    fn clean(&mut self) {
        unsafe { self.dispose() };
    }
}

#![allow(dead_code)]

type LineNumber = usize;
type ErrorMessage = String;

pub trait BoltError {
    fn new(message: ErrorMessage, kind: Option<BoltErrorType>, line: Option<LineNumber>) -> Self;
    fn get_type(&self) -> BoltErrorType;
    fn get_message(&self) -> ErrorMessage;
}

#[derive(Debug, Clone)]
pub enum BoltErrorType {
    GENERIC,
    INTERNAL,
    PARSE,
    EOF,
    EVAL,
}

#[derive(Debug)]
pub struct ParseError {
    message: ErrorMessage,
    line: Option<LineNumber>,
    kind: BoltErrorType,
}

impl BoltError for ParseError {
    fn new(message: ErrorMessage, kind: Option<BoltErrorType>, line: Option<LineNumber>) -> Self {
        let error_kind = kind.unwrap_or(BoltErrorType::PARSE);
        return Self {
            message: message,
            kind: error_kind,
            line: line,
        };
    }

    fn get_message(&self) -> ErrorMessage {
        return self.message.clone();
    }

    fn get_type(&self) -> BoltErrorType {
        return self.kind.clone();
    }
}

#[derive(Debug, Clone)]
pub struct EvaluatorError {
    message: ErrorMessage,
    line: Option<LineNumber>,
    kind: BoltErrorType,
}

impl BoltError for EvaluatorError {
    fn new(message: ErrorMessage, kind: Option<BoltErrorType>, line: Option<LineNumber>) -> Self {
        let error_kind = kind.unwrap_or(BoltErrorType::EVAL);
        return Self {
            message: message,
            kind: error_kind,
            line: line,
        };
    }

    fn get_message(&self) -> ErrorMessage {
        return self.message.clone();
    }

    fn get_type(&self) -> BoltErrorType {
        return self.kind.clone();
    }
}

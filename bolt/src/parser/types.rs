pub enum Precedences {
    LOWEST = 0,
    EQUALS = 1,
    LESSGREATER = 2,
    SUM = 3,
    PRODUCT = 4,
    PREFIX = 5,
    CALL = 6,
}

pub type PrecedenceValue = usize;

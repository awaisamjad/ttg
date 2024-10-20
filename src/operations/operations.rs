#[repr(u8)]
#[derive(PartialEq)]
pub enum Value {
    TRUE,
    FALSE,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::TRUE => write!(f, "TRUE"),
            Value::FALSE => write!(f, "FALSE"),
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub fields: Vec<Value>,
}

#[derive(Debug)]
pub enum OperationError {
    LengthsNotEqual,
    ZeroLength,
}

pub trait OPERATIONS {
    fn and(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn or(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn not(self) -> Result<Statement, OperationError>;
    fn xor(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn xnor(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn if_then(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn r#if(self, s2: &Statement) -> Result<Statement, OperationError>;
    fn biconditional(self, s2: &Statement) -> Result<Statement, OperationError>;
}

impl OPERATIONS for Statement {
    fn and(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let mut result_fields = Vec::new();

        for (field1, field2) in self.fields.iter().zip(s2.fields.iter()) {
            let result_field = if *field1 == Value::TRUE && *field2 == Value::TRUE {
                Value::TRUE
            } else {
                Value::FALSE
            };
            result_fields.push(result_field);
        }

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn not(self) -> Result<Statement, OperationError> {
        let result_fields: Vec<Value> = self
            .fields
            .iter()
            .map(|field| {
                if field == &Value::TRUE {
                    Value::FALSE
                } else {
                    Value::TRUE
                }
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn or(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn xor(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::FALSE,
                (Value::TRUE, Value::TRUE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn xnor(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::TRUE,
                (Value::TRUE, Value::TRUE) => Value::TRUE,
                _ => Value::FALSE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn if_then(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::TRUE, Value::FALSE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn r#if(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::TRUE) => Value::FALSE,
                _ => Value::TRUE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }

    fn biconditional(self, s2: &Statement) -> Result<Statement, OperationError> {
        if self.fields.len() != s2.fields.len() {
            return Err(OperationError::LengthsNotEqual);
        }

        let result_fields = self
            .fields
            .iter()
            .zip(&s2.fields)
            .map(|(value1, value2)| match (value1, value2) {
                (Value::FALSE, Value::FALSE) => Value::TRUE,
                (Value::TRUE, Value::TRUE) => Value::TRUE,
                _ => Value::FALSE,
            })
            .collect();

        Ok(Statement {
            fields: result_fields,
        })
    }
}

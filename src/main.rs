const AND: (&str, &str) = ("^", "AND");

const OR: (&str, &str) = ("v", "OR");

#[derive(Debug)]
enum OPERATION_ERROR {
    LENGTHS_NOT_EQUAL,
    ZERO_LENGTH
}

#[repr(u8)]
#[derive(PartialEq)]
enum Value {
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

trait OPERATIONS {
    fn and(self, s2: Statement) -> Result<Statement, OPERATION_ERROR>;

    fn or(self, s2: Statement) -> Statement;
    fn not(self) -> Statement;
}

#[derive(Debug)]
struct Statement {
    fields: Vec<Value>,
}

impl Statement {
    // fn iter(&self) -> impl Iterator<Item = &Value> {
    //     vec![&self.x1, &self.x2, &self.x3, &self.x4].into_iter()
    // }
}

impl OPERATIONS for Statement {
    fn and(self, s2: Statement) -> Result<Statement, OPERATION_ERROR> {

        if self.fields.len() == 0 || s2.fields.len() == 0 {
            return Err(OPERATION_ERROR::ZERO_LENGTH);
        }

        if self.fields.len() != s2.fields.len() {
            return Err(OPERATION_ERROR::LENGTHS_NOT_EQUAL);
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

    fn not(self) -> Self {
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

        Statement {
            fields: result_fields,
        }
    }

    fn or(self, s2: Statement) -> Statement {
        todo!()
    }
}

//? allows indexing of a struct
// impl std::ops::Index<usize> for Statement {
//     type Output = Value;

//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.x1,
//             1 => &self.x2,
//             2 => &self.x3,
//             3 => &self.x4,
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }

fn main() {
    let p1 = Statement {
        fields: vec![Value::TRUE, Value::TRUE, Value::FALSE, Value::FALSE],
    };

    let p2 = Statement {
        fields: vec![Value::TRUE, Value::FALSE, Value::TRUE, Value::FALSE, Value::TRUE],
    };

    let result = p1.and(p2);
    match result {
        Ok(value) => println!("{:?}", value),
        Err(err) => println!("{:?}. Check if statements have the same length", err)
    }
    // let result = p1.not();

    // for value in result.fields {
    //     println!("{:?}", value);
    // }
}

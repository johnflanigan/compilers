use crate::checked_grammar::CheckedDec;
use crate::checked_grammar::{CheckedExp, CheckedLValue, CheckedProgram, CheckedTopLevelDec};
use crate::common::{InfixSourceOp, Label, Symbol};
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Void,
    Str(String),
    Int(i64),
    Array(Vec<Value>),
    Record(Vec<(String, Value)>),
}

fn find_function(prog: &CheckedProgram, name: Label) -> &CheckedTopLevelDec {
    let mut v: Vec<_> = prog
        .dec_list
        .iter()
        .filter_map(|f| match f {
            dec @ CheckedTopLevelDec::FunDec { .. } => {
                if name == Label::Main {
                    Some(dec)
                } else {
                    None
                }
            }
        })
        .collect();
    assert_eq!(v.len(), 1);
    v.pop().unwrap()
}

pub fn eval(prog: &CheckedProgram) -> Value {
    let main = find_function(prog, Label::Main);
    eval_fn(prog, main, vec![])
}

pub struct Env {
    values: HashMap<Symbol, Value>,
}

impl Env {
    fn insert(&mut self, s: Symbol, v: Value) {
        self.values.insert(s, v);
    }

    fn get(&mut self, s: Symbol) -> Value {
        self.values
            .get(&s)
            .expect("Use of undefined Symbol")
            .clone()
    }
}

pub fn eval_fn(prog: &CheckedProgram, fun: &CheckedTopLevelDec, arguments: Vec<Value>) -> Value {
    match fun {
        CheckedTopLevelDec::FunDec { args, body, .. } => {
            assert_eq!(args.len(), arguments.len());
            let mut env = Env {
                values: args.iter().copied().zip(arguments.into_iter()).collect(),
            };
            eval_exp(prog, &mut env, body).expect("Break broke out of function improperly")
        }
    }
}

fn eval_lvalue(prog: &CheckedProgram, env: &mut Env, exp: &CheckedLValue) -> Value {
    match exp {
        CheckedLValue::Id { name } => dbg!(env.get(*name)),
        CheckedLValue::Subscript { array, index } => {
            let index: usize = match eval_exp(prog, env, index).expect("Broke from array index") {
                Value::Int(index) => index.try_into().unwrap(),
                _ => panic!("Subscript index is not an int"),
            };
            match eval_lvalue(prog, env, array) {
                Value::Array(v) => v.get(index).unwrap().clone(),
                _ => panic!("Subscripting non array value"),
            }
        }
        CheckedLValue::FieldExp { record, field } => match dbg!(eval_lvalue(prog, env, record)) {
            Value::Record(feilds) => {
                let mut value = None;
                for (s, v) in feilds.into_iter() {
                    if &s == field {
                        value = Some(v);
                    }
                }
                value.expect("Accessing non-existent field")
            }
            _ => panic!("Accessing fields of non-record"),
        },
    }
}

fn eval_lvalue_ref<'a>(
    prog: &CheckedProgram,
    env: &'a mut Env,
    exp: &CheckedLValue,
) -> &'a mut Value {
    match exp {
        CheckedLValue::Id { name } => env.values.get_mut(name).unwrap(),
        CheckedLValue::Subscript { array, index } => {
            let i: usize =
                match eval_exp(prog, env, index).expect("Index had 'break' in assignment") {
                    Value::Int(i) => i.try_into().unwrap(),
                    _ => panic!("Non-Int index in array"),
                };
            match eval_lvalue_ref(prog, env, array) {
                Value::Array(vec) => vec.get_mut(i).expect("Index out of range"),
                _ => panic!("Index into non-vec"),
            }
        }
        CheckedLValue::FieldExp { record, field } => match eval_lvalue_ref(prog, env, record) {
            Value::Record(fields) => {
                let mut value = None;
                for (s, v) in fields.iter_mut() {
                    if s.clone() == field.clone() {
                        value = Some(v);
                    }
                }
                value.expect("Accessing non-existent field")
            }
            _ => panic!("Index into non-vec"),
        },
    }
}

pub fn eval_exp(prog: &CheckedProgram, env: &mut Env, exp: &CheckedExp) -> Option<Value> {
    match exp {
        CheckedExp::Break => None,
        CheckedExp::IntLit { value } => Some(Value::Int((*value).into())),
        CheckedExp::StringLit { value } => Some(Value::Str(value.clone())),
        CheckedExp::LValue { lvalue } => Some(eval_lvalue(prog, env, dbg!(lvalue))),
        CheckedExp::Sequence { sequence } => {
            let mut result = Value::Void;
            for exp in sequence {
                result = eval_exp(prog, env, exp)?;
            }
            Some(result)
        }
        CheckedExp::Negate { exp } => match eval_exp(prog, env, exp) {
            Some(Value::Int(value)) => Some(Value::Int(-value)),
            None => None,
            _ => panic!("Negate doesn't contain int value"),
        },
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::Add,
            right,
        } => {
            let left_value = if let Value::Int(v) = dbg!(eval_exp(prog, env, left)?) {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(Value::Int(left_value + right_value))
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::Subtract,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(Value::Int(left_value - right_value))
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::Multiply,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(Value::Int(left_value * right_value))
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::Divide,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(Value::Int(left_value.checked_div(right_value)?))
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::Equal,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(if left_value == right_value {
                Value::Int(1)
            } else {
                Value::Int(0)
            })
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::NotEqual,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(if left_value != right_value {
                Value::Int(1)
            } else {
                Value::Int(0)
            })
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::GreaterThan,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(if left_value > right_value {
                Value::Int(1)
            } else {
                Value::Int(0)
            })
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::LessThan,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(if left_value < right_value {
                Value::Int(1)
            } else {
                Value::Int(0)
            })
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::GreaterThanEqual,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(if left_value >= right_value {
                Value::Int(1)
            } else {
                Value::Int(0)
            })
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::LessThanEqual,
            right,
        } => {
            let left_value = if let Value::Int(v) = eval_exp(prog, env, left)? {
                v
            } else {
                panic!("evaluated left side of infix op to non Int type")
            };
            let right_value = if let Value::Int(v) = eval_exp(prog, env, right)? {
                v
            } else {
                panic!("evaluated right side of infix op to non int type")
            };
            Some(if left_value <= right_value {
                Value::Int(1)
            } else {
                Value::Int(0)
            })
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::And,
            right,
        } => {
            if let Value::Int(0) = eval_exp(prog, env, left)? {
                Some(Value::Int(0))
            } else {
                eval_exp(prog, env, right)
            }
        }
        CheckedExp::Infix {
            left,
            op: InfixSourceOp::Or,
            right,
        } => {
            if let Value::Int(1) = eval_exp(prog, env, left)? {
                Some(Value::Int(1))
            } else {
                eval_exp(prog, env, right)
            }
        }
        CheckedExp::ArrayCreate {
            length,
            initial_value,
        } => {
            let len = if let Value::Int(v) = eval_exp(prog, env, length)? {
                v
            } else {
                panic!("length in array create not int")
            };
            let init = eval_exp(prog, env, initial_value)?;
            Some(Value::Array(
                iter::repeat(init).take(len.try_into().unwrap()).collect(),
            ))
        }
        CheckedExp::RecordCreate { fields } => {
            let mut f = vec![];
            for (s, v) in fields.iter() {
                f.push((s.clone(), eval_exp(prog, env, v)?));
            }
            Some(Value::Record(f))
        }
        CheckedExp::Assign { left, right } => {
            let value = eval_exp(prog, env, right)?;
            *eval_lvalue_ref(prog, env, left) = value;
            Some(Value::Void)
        }
        CheckedExp::IfThenElse {
            if_exp,
            then_exp,
            else_exp,
        } => {
            if let Value::Int(0) = eval_exp(prog, env, if_exp)? {
                // 0 is false-y
                eval_exp(prog, env, else_exp)
            } else {
                eval_exp(prog, env, then_exp)
            }
        }
        CheckedExp::IfThen { if_exp, then_exp } => {
            Some(if let Value::Int(0) = eval_exp(prog, env, if_exp)? {
                Value::Void
            } else {
                eval_exp(prog, env, then_exp)?;
                Value::Void
            })
        }
        CheckedExp::While { while_exp, do_exp } => {
            while let Value::Int(n) = eval_exp(prog, env, while_exp)? {
                if n == 0 {
                    break;
                }
                match eval_exp(prog, env, do_exp) {
                    Some(_) => continue,
                    None => break,
                }
            }
            Some(Value::Void)
        }
        CheckedExp::For {
            id,
            for_exp,
            to_exp,
            do_exp,
        } => {
            let start_value = match eval_exp(prog, env, for_exp)? {
                Value::Int(i) => Value::Int(i),
                _ => panic!("for_exp doesn't evaluate to int"),
            };
            let end_value = match eval_exp(prog, env, to_exp)? {
                Value::Int(i) => i,
                _ => panic!("to_exp doesn't evaluate to int"),
            };
            env.insert(*id, start_value);
            while let Value::Int(i) = env.get(*id) {
                if i > end_value {
                    break;
                }
                match eval_exp(prog, env, do_exp) {
                    Some(_) => (),
                    None => break,
                }
                env.insert(*id, Value::Int(i + 1))
            }
            Some(Value::Void)
        }
        CheckedExp::Let { let_exp, in_exp } => {
            for dec in let_exp.iter() {
                match dec {
                    CheckedDec::VarDec { name, value } => {
                        let v = eval_exp(prog, env, value)?;
                        env.insert(*name, v);
                    }
                }
            }
            eval_exp(prog, env, in_exp)
        }
        CheckedExp::Call {
            function_name,
            args,
        } => {
            let arg_values: Vec<_> = args
                .iter()
                .map(|exp| eval_exp(prog, env, exp))
                .collect::<Option<_>>()?;
            let fun = find_function(prog, *function_name);
            Some(eval_fn(prog, fun, arg_values))
        }
    }
}

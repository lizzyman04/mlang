use crate::core::interpreter::env::Environment;
use crate::core::lexer::rules::infer_type;
use crate::core::lexer::symbol::comparison::ComparisonSymbolKind;
use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::{Expression, Type};
use Expression::*;

pub fn evaluate(expr: Expression, env: &mut Environment) -> Result<Expression, String> {
    match expr {
        IntLiteral(_) | DecLiteral(_) | TxtLiteral(_) | BoolLiteral(_) => Ok(expr),

        ArrayLiteral(elems) => {
            let evaluated = elems
                .into_iter()
                .map(|e| evaluate(e, env))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ArrayLiteral(evaluated))
        }

        Identifier(name) => env
            .get(&name)
            .map(|(_, v)| v.clone())
            .ok_or_else(|| format!("Undefined variable '{}'", name)),

        Binary { left, operator, right } => {
            let lv = evaluate(*left, env)?;
            let rv = evaluate(*right, env)?;
            eval_binary(lv, rv, &operator)
        }

        Unary { operator, operand } => {
            let val = evaluate(*operand, env)?;
            match (&operator.kind, val) {
                (TokenKind::SimpleSymbol(SimpleSymbolKind::Not), BoolLiteral(b)) => {
                    Ok(BoolLiteral(!b))
                }
                (TokenKind::SimpleSymbol(SimpleSymbolKind::Minus), IntLiteral(i)) => {
                    Ok(IntLiteral(-i))
                }
                (TokenKind::SimpleSymbol(SimpleSymbolKind::Minus), DecLiteral(f)) => {
                    Ok(DecLiteral(-f))
                }
                _ => Err("Invalid unary operation".to_string()),
            }
        }

        Logical { left, operator, right } => {
            let lv = evaluate(*left, env)?;
            match (&operator.kind, &lv) {
                (TokenKind::SimpleSymbol(SimpleSymbolKind::And), BoolLiteral(false)) => {
                    Ok(BoolLiteral(false))
                }
                (TokenKind::SimpleSymbol(SimpleSymbolKind::Or), BoolLiteral(true)) => {
                    Ok(BoolLiteral(true))
                }
                (
                    TokenKind::SimpleSymbol(SimpleSymbolKind::And | SimpleSymbolKind::Or),
                    BoolLiteral(_),
                ) => match evaluate(*right, env)? {
                    BoolLiteral(b) => Ok(BoolLiteral(b)),
                    _ => Err("Logical operators require boolean operands".to_string()),
                },
                _ => Err("Logical operators require boolean operands".to_string()),
            }
        }

        ArrayAccess { array, index } => {
            let arr = evaluate(*array, env)?;
            let idx = evaluate(*index, env)?;
            match (arr, idx) {
                (ArrayLiteral(elems), IntLiteral(i)) => {
                    if i < 0 || i as usize >= elems.len() {
                        Err(format!(
                            "Index {} out of bounds for array of length {}",
                            i,
                            elems.len()
                        ))
                    } else {
                        Ok(elems[i as usize].clone())
                    }
                }
                (_, IntLiteral(_)) => Err("Array access on non-array value".to_string()),
                _ => Err("Array index must be an integer".to_string()),
            }
        }

        MethodCall { object, method, args } => {
            let maybe_var = match &*object {
                Identifier(n) => Some(n.clone()),
                _ => None,
            };
            // Evaluate args before object so side-effects (e.g. nested pops) run first
            let evaled_args = args
                .into_iter()
                .map(|a| evaluate(a, env))
                .collect::<Result<Vec<_>, _>>()?;
            let obj_val = evaluate(*object, env)?;

            match method.as_str() {
                "len" => match obj_val {
                    ArrayLiteral(elems) => Ok(IntLiteral(elems.len() as i64)),
                    _ => Err("'len' requires an array".to_string()),
                },
                "contains" => match obj_val {
                    ArrayLiteral(elems) => {
                        let target = evaled_args
                            .into_iter()
                            .next()
                            .ok_or("'contains' requires one argument")?;
                        Ok(BoolLiteral(elems.iter().any(|e| exprs_equal(e, &target))))
                    }
                    _ => Err("'contains' requires an array".to_string()),
                },
                "slice" => match obj_val {
                    ArrayLiteral(elems) => {
                        let mut it = evaled_args.into_iter();
                        let start = to_index(it.next().ok_or("'slice' requires two arguments")?)?;
                        let end = to_index(it.next().ok_or("'slice' requires two arguments")?)?;
                        if start > elems.len() || end > elems.len() || start > end {
                            return Err(format!(
                                "slice {}..{} out of bounds for array of length {}",
                                start,
                                end,
                                elems.len()
                            ));
                        }
                        Ok(ArrayLiteral(elems[start..end].to_vec()))
                    }
                    _ => Err("'slice' requires an array".to_string()),
                },
                m @ ("push" | "pop" | "clear") => {
                    let var_name = maybe_var.ok_or_else(|| {
                        format!("'{}' requires a mutable variable target", m)
                    })?;
                    let arr_type = env
                        .get(&var_name)
                        .ok_or_else(|| format!("Undefined variable '{}'", var_name))?
                        .0
                        .clone();
                    let inner_type = match &arr_type {
                        Type::Array(inner) => *inner.clone(),
                        _ => return Err(format!("'{}' is not an array", var_name)),
                    };
                    match (m, obj_val) {
                        ("push", ArrayLiteral(mut elems)) => {
                            let arg = evaled_args
                                .into_iter()
                                .next()
                                .ok_or("'push' requires one argument")?;
                            elems.push(coerce_value(&inner_type, arg)?);
                            let new_len = elems.len() as i64;
                            env.set(var_name, arr_type, ArrayLiteral(elems));
                            Ok(IntLiteral(new_len))
                        }
                        ("pop", ArrayLiteral(mut elems)) => {
                            if elems.is_empty() {
                                return Err("Cannot pop from an empty array".to_string());
                            }
                            let last = elems.pop().unwrap();
                            env.set(var_name, arr_type, ArrayLiteral(elems));
                            Ok(last)
                        }
                        ("clear", ArrayLiteral(_)) => {
                            env.set(var_name, arr_type, ArrayLiteral(vec![]));
                            Ok(IntLiteral(0))
                        }
                        _ => Err(format!("'{}' is not an array", var_name)),
                    }
                }
                m => Err(format!("Unknown method '{}'", m)),
            }
        }

        FnCall { name, .. } => Err(format!("Function '{}' not yet callable", name)),
    }
}

fn eval_binary(lv: Expression, rv: Expression, op: &crate::core::lexer::token::Token) -> Result<Expression, String> {
    use TokenKind::*;
    use SimpleSymbolKind::*;
    match (lv, rv, &op.kind) {
        // int arithmetic
        (IntLiteral(a), IntLiteral(b), SimpleSymbol(Plus)) => Ok(IntLiteral(a + b)),
        (IntLiteral(a), IntLiteral(b), SimpleSymbol(Minus)) => Ok(IntLiteral(a - b)),
        (IntLiteral(a), IntLiteral(b), SimpleSymbol(Star)) => Ok(IntLiteral(a * b)),
        (IntLiteral(a), IntLiteral(b), SimpleSymbol(Slash)) => {
            if b == 0 { Err("Division by zero".to_string()) } else { Ok(IntLiteral(a / b)) }
        }
        // dec arithmetic
        (DecLiteral(a), DecLiteral(b), SimpleSymbol(Plus)) => Ok(DecLiteral(a + b)),
        (DecLiteral(a), DecLiteral(b), SimpleSymbol(Minus)) => Ok(DecLiteral(a - b)),
        (DecLiteral(a), DecLiteral(b), SimpleSymbol(Star)) => Ok(DecLiteral(a * b)),
        (DecLiteral(a), DecLiteral(b), SimpleSymbol(Slash)) => {
            if b == 0.0 { Err("Division by zero".to_string()) } else { Ok(DecLiteral(a / b)) }
        }
        // mixed int/dec (promote to dec)
        (IntLiteral(a), DecLiteral(b), SimpleSymbol(Plus)) => Ok(DecLiteral(a as f64 + b)),
        (DecLiteral(a), IntLiteral(b), SimpleSymbol(Plus)) => Ok(DecLiteral(a + b as f64)),
        (IntLiteral(a), DecLiteral(b), SimpleSymbol(Minus)) => Ok(DecLiteral(a as f64 - b)),
        (DecLiteral(a), IntLiteral(b), SimpleSymbol(Minus)) => Ok(DecLiteral(a - b as f64)),
        (IntLiteral(a), DecLiteral(b), SimpleSymbol(Star)) => Ok(DecLiteral(a as f64 * b)),
        (DecLiteral(a), IntLiteral(b), SimpleSymbol(Star)) => Ok(DecLiteral(a * b as f64)),
        (IntLiteral(a), DecLiteral(b), SimpleSymbol(Slash)) => {
            if b == 0.0 { Err("Division by zero".to_string()) } else { Ok(DecLiteral(a as f64 / b)) }
        }
        (DecLiteral(a), IntLiteral(b), SimpleSymbol(Slash)) => {
            if b == 0 { Err("Division by zero".to_string()) } else { Ok(DecLiteral(a / b as f64)) }
        }
        // string concatenation
        (TxtLiteral(a), TxtLiteral(b), SimpleSymbol(Plus)) => Ok(TxtLiteral(format!("{}{}", a, b))),
        // int comparisons
        (IntLiteral(a), IntLiteral(b), ComparisonSymbol(k)) => Ok(BoolLiteral(cmp_int(a, b, k))),
        (DecLiteral(a), DecLiteral(b), ComparisonSymbol(k)) => Ok(BoolLiteral(cmp_dec(a, b, k))),
        (IntLiteral(a), DecLiteral(b), ComparisonSymbol(k)) => Ok(BoolLiteral(cmp_dec(a as f64, b, k))),
        (DecLiteral(a), IntLiteral(b), ComparisonSymbol(k)) => Ok(BoolLiteral(cmp_dec(a, b as f64, k))),
        (TxtLiteral(a), TxtLiteral(b), ComparisonSymbol(ComparisonSymbolKind::EqualEqual)) => Ok(BoolLiteral(a == b)),
        (TxtLiteral(a), TxtLiteral(b), ComparisonSymbol(ComparisonSymbolKind::NotEqual)) => Ok(BoolLiteral(a != b)),
        (BoolLiteral(a), BoolLiteral(b), ComparisonSymbol(ComparisonSymbolKind::EqualEqual)) => Ok(BoolLiteral(a == b)),
        (BoolLiteral(a), BoolLiteral(b), ComparisonSymbol(ComparisonSymbolKind::NotEqual)) => Ok(BoolLiteral(a != b)),
        // math symbols → helpful error
        (_, _, MathSymbol(k)) => Err(format!(
            "Math symbol '{}' at line {} is not supported directly. Use `math(...)` instead.",
            k.to_char(), op.line
        )),
        _ => Err(format!("Invalid binary operation or type mismatch at line {}", op.line)),
    }
}

fn cmp_int(a: i64, b: i64, k: &ComparisonSymbolKind) -> bool {
    match k {
        ComparisonSymbolKind::EqualEqual => a == b,
        ComparisonSymbolKind::NotEqual => a != b,
        ComparisonSymbolKind::Less => a < b,
        ComparisonSymbolKind::Greater => a > b,
        ComparisonSymbolKind::LessEqual => a <= b,
        ComparisonSymbolKind::GreaterEqual => a >= b,
    }
}

fn cmp_dec(a: f64, b: f64, k: &ComparisonSymbolKind) -> bool {
    match k {
        ComparisonSymbolKind::EqualEqual => a == b,
        ComparisonSymbolKind::NotEqual => a != b,
        ComparisonSymbolKind::Less => a < b,
        ComparisonSymbolKind::Greater => a > b,
        ComparisonSymbolKind::LessEqual => a <= b,
        ComparisonSymbolKind::GreaterEqual => a >= b,
    }
}

fn to_index(expr: Expression) -> Result<usize, String> {
    match expr {
        IntLiteral(i) if i >= 0 => Ok(i as usize),
        IntLiteral(i) => Err(format!("Array index cannot be negative: {}", i)),
        _ => Err("Array index must be an integer".to_string()),
    }
}

pub fn coerce_value(expected: &Type, value: Expression) -> Result<Expression, String> {
    match expected {
        Type::Int => match value {
            IntLiteral(_) => Ok(value),
            _ => type_err(expected, &value),
        },
        Type::Dec => match value {
            DecLiteral(_) => Ok(value),
            IntLiteral(i) => Ok(DecLiteral(i as f64)),
            _ => type_err(expected, &value),
        },
        Type::Txt => match value {
            TxtLiteral(_) => Ok(value),
            _ => type_err(expected, &value),
        },
        Type::Bool => match value {
            BoolLiteral(_) => Ok(value),
            _ => type_err(expected, &value),
        },
        Type::Array(inner) => match value {
            ArrayLiteral(elems) => {
                let coerced = elems
                    .into_iter()
                    .map(|e| coerce_value(inner, e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ArrayLiteral(coerced))
            }
            _ => type_err(expected, &value),
        },
        Type::Void => Ok(value),
    }
}

fn type_err(expected: &Type, got: &Expression) -> Result<Expression, String> {
    Err(format!("Expected '{}', got '{}'", expected, infer_type(got)))
}

fn exprs_equal(a: &Expression, b: &Expression) -> bool {
    match (a, b) {
        (IntLiteral(x), IntLiteral(y)) => x == y,
        (DecLiteral(x), DecLiteral(y)) => x == y,
        (TxtLiteral(x), TxtLiteral(y)) => x == y,
        (BoolLiteral(x), BoolLiteral(y)) => x == y,
        _ => false,
    }
}

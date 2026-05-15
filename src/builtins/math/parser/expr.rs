use super::ast::{BinaryOp, ExprNode};
use super::precedence::{get_precedence, is_right_associative};
use crate::builtins::math::lexer::MathToken;

/// Pratt parser with precedence climbing.
///
/// Handles implicit multiplication: when an Ident, Func, or LParen immediately
/// follows a complete expression (no operator in between), a virtual Mul is
/// inserted with precedence 2. The implicit token is NOT consumed here — the
/// next parse_primary call reads it normally.
pub fn parse_expr(tokens: &[MathToken], pos: &mut usize, min_prec: u8) -> Result<ExprNode, String> {
    let mut lhs = parse_primary(tokens, pos)?;

    loop {
        // Determine the next operator — explicit or implicit.
        let (op, is_implicit) = match tokens.get(*pos) {
            Some(MathToken::Plus) => (BinaryOp::Add, false),
            Some(MathToken::Minus) => (BinaryOp::Sub, false),
            Some(MathToken::Star) => (BinaryOp::Mul, false),
            Some(MathToken::Slash) => (BinaryOp::Div, false),
            Some(MathToken::Caret) => (BinaryOp::Pow, false),
            // Implicit multiplication: next token can begin a primary
            Some(MathToken::Ident(_)) | Some(MathToken::Func(_)) | Some(MathToken::LParen) => {
                (BinaryOp::Mul, true)
            }
            _ => break,
        };

        let prec = get_precedence(&op);
        if prec < min_prec {
            break;
        }

        // Only consume the token for explicit operators.
        if !is_implicit {
            *pos += 1;
        }

        // Right-associative ops climb at the same level; left-assoc climb one higher.
        let rhs_min_prec = if is_right_associative(&op) { prec } else { prec + 1 };
        let rhs = parse_expr(tokens, pos, rhs_min_prec)?;

        lhs = ExprNode::Binary {
            op,
            left: Box::new(lhs),
            right: Box::new(rhs),
        };
    }

    Ok(lhs)
}

fn parse_primary(tokens: &[MathToken], pos: &mut usize) -> Result<ExprNode, String> {
    match tokens.get(*pos) {
        Some(MathToken::Num(n)) => {
            let val = *n;
            *pos += 1;
            Ok(ExprNode::Num(val))
        }

        Some(MathToken::Ident(name)) => {
            let name = name.clone();
            *pos += 1;
            Ok(ExprNode::Var(name))
        }

        Some(MathToken::Func(name)) => {
            let name = name.clone();
            *pos += 1;
            if !matches!(tokens.get(*pos), Some(MathToken::LParen)) {
                return Err(format!("expected '(' after function '{}'", name));
            }
            *pos += 1; // consume '('
            let arg = parse_expr(tokens, pos, 0)?;
            if !matches!(tokens.get(*pos), Some(MathToken::RParen)) {
                return Err(format!("expected ')' after argument to '{}'", name));
            }
            *pos += 1; // consume ')'
            Ok(ExprNode::FuncCall { name, arg: Box::new(arg) })
        }

        Some(MathToken::Minus) => {
            *pos += 1;
            // Parse with high precedence so unary minus binds tightly.
            let operand = parse_primary(tokens, pos)?;
            Ok(ExprNode::UnaryMinus(Box::new(operand)))
        }

        Some(MathToken::LParen) => {
            *pos += 1;
            let inner = parse_expr(tokens, pos, 0)?;
            if !matches!(tokens.get(*pos), Some(MathToken::RParen)) {
                return Err("expected ')'".to_string());
            }
            *pos += 1; // consume ')'
            Ok(inner)
        }

        Some(tok) => Err(format!("unexpected token '{:?}' in expression", tok)),
        None => Err("unexpected end of expression".to_string()),
    }
}

use crate::builtins::math::parser::ExprNode;

/// Alias kept thin — normalizer works entirely on the parser's ExprNode tree.
pub type NormalizedNode = ExprNode;

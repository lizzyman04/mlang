// use crate::core::parser::ast::ASTNode;
// use crate::core::parser::parser::Parser;
// use crate::core::lexer::token::TokenKind;

// pub fn parse_statement(parser: &mut Parser) -> Result<ASTNode, String> {
//     if let Some(token) = parser.peek() {
//         match &token.kind {
//             TokenKind::Keyword(kw) if kw == "main" => parse_function_decl(parser),
//             TokenKind::Keyword(kw) if kw == "print" => parse_print_stmt(parser),
//             TokenKind::Keyword(kw) if kw == "int" || kw == "dec" || kw == "txt" => parse_var_decl(parser),
//             _ => Err(format!(
//                 "Unexpected token {:?} at line {}, column {}",
//                 token.kind, token.line, token.column
//             )),
//         }
//     } else {
//         Err("Unexpected end of input".to_string())
//     }
// }

// fn parse_function_decl(parser: &mut Parser) -> Result<ASTNode, String> {
//     parser.consume_keyword("main")?;
//     parser.consume(&TokenKind::LeftParen)?;
//     parser.consume(&TokenKind::RightParen)?;
//     parser.consume(&TokenKind::LeftBrace)?;

//     let mut body = Vec::new();
//     while let Some(token) = parser.peek() {
//         if let TokenKind::RightBrace = token.kind {
//             break;
//         }
//         let stmt = parse_statement(parser)?;
//         body.push(stmt);
//     }

//     parser.consume(&TokenKind::RightBrace)?;

//     Ok(ASTNode::FunctionDecl {
//         name: "main".to_string(),
//         body,
//     })
// }

// fn parse_print_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
//     parser.consume_keyword("print")?;
//     parser.consume(&TokenKind::LeftParen)?;
//     let expr = parse_expression(parser)?;
//     parser.consume(&TokenKind::RightParen)?;
//     parser.consume(&TokenKind::Semicolon)?;

//     Ok(ASTNode::PrintStmt {
//         expression: Box::new(expr),
//     })
// }

// pub fn parse_expression(parser: &mut Parser) -> Result<ASTNode, String> {
//     if let Some(token) = parser.advance() {
//         match &token.kind {
//             TokenKind::Int(i) => Ok(ASTNode::IntLiteral(*i)),
//             TokenKind::Txt(s) => Ok(ASTNode::TxtLiteral(s.clone())),
//             TokenKind::Identifier(id) => Ok(ASTNode::Identifier(id.clone())),
//             _ => Err(format!(
//                 "Expected literal but found {:?} at line {}, column {}",
//                 token.kind, token.line, token.column
//             )),
//         }
//     } else {
//         Err("Unexpected end of input in expression.".to_string())
//     }
// }

// fn parse_var_decl(parser: &mut Parser) -> Result<ASTNode, String> {
//     let var_type_token = parser.advance().ok_or("Expected type keyword")?;
//     let var_type = match &var_type_token.kind {
//         TokenKind::Keyword(t) => t.clone(),
//         _ => return Err("Expected type keyword".to_string()),
//     };

//     let name_token = parser.advance().ok_or("Expected variable name")?;
//     let name = match &name_token.kind {
//         TokenKind::Identifier(n) => n.clone(),
//         _ => return Err("Expected identifier for variable name".to_string()),
//     };

//     parser.consume(&TokenKind::Equal)?;
//     let expr = parse_expression(parser)?;
//     parser.consume(&TokenKind::Semicolon)?;

//     Ok(ASTNode::VarDecl {
//         var_type,
//         name,
//         value: Box::new(expr),
//     })
// }

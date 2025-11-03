#[cfg(test)]
mod parser_tests {
    use rlox::transpiler::chunk::{Chunk, OpCode};
    use rlox::transpiler::parser::compile;

    // Helper function to compile and return the chunk
    fn compile_expression(source: &str) -> (Chunk, bool) {
        let mut source_string = source.to_string();
        let mut chunk = Chunk::new();
        let success = compile(&mut source_string, &mut chunk);
        (chunk, success)
    }

    // Helper to check if chunk contains specific opcodes
    fn has_opcode(chunk: &Chunk, opcode: OpCode) -> bool {
        chunk.code.iter().any(|byte| byte.byte == opcode as u8)
    }

    #[test]
    fn test_simple_number() {
        let (chunk, success) = compile_expression("42");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpConstant));
        assert!(has_opcode(&chunk, OpCode::OpReturn));
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_addition() {
        let (chunk, success) = compile_expression("1 + 2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpConstant));
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpReturn));
        assert_eq!(chunk.constant_pool.len(), 2);
    }

    #[test]
    fn test_subtraction() {
        let (chunk, success) = compile_expression("5 - 3");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpSubtract));
    }

    #[test]
    fn test_multiplication() {
        let (chunk, success) = compile_expression("4 * 3");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
    }

    #[test]
    fn test_division() {
        let (chunk, success) = compile_expression("10 / 2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpDivide));
    }

    #[test]
    fn test_negation() {
        let (chunk, success) = compile_expression("-5");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpConstant));
        assert!(has_opcode(&chunk, OpCode::OpNegate));
    }

    #[test]
    fn test_grouping() {
        let (chunk, success) = compile_expression("(1 + 2)");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_precedence_multiplication_before_addition() {
        let (chunk, success) = compile_expression("2 + 3 * 4");
        assert!(success);
        // Should compile as: 2, 3, 4, *, +
        // This means multiplication happens before addition
        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();
        let add_pos = code_bytes.iter().position(|&b| b == OpCode::OpAdd as u8).unwrap();
        let mul_pos = code_bytes.iter().position(|&b| b == OpCode::OpMultiply as u8).unwrap();
        assert!(mul_pos < add_pos, "Multiplication should come before addition");
    }

    #[test]
    fn test_precedence_grouping_overrides() {
        let (chunk, success) = compile_expression("(2 + 3) * 4");
        assert!(success);
        // Should compile as: 2, 3, +, 4, *
        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();
        let add_pos = code_bytes.iter().position(|&b| b == OpCode::OpAdd as u8).unwrap();
        let mul_pos = code_bytes.iter().position(|&b| b == OpCode::OpMultiply as u8).unwrap();
        assert!(add_pos < mul_pos, "Addition should come before multiplication due to grouping");
    }

    #[test]
    fn test_complex_expression() {
        let (chunk, success) = compile_expression("1 + 2 * 3 - 4 / 2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpSubtract));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
        assert!(has_opcode(&chunk, OpCode::OpDivide));
        assert_eq!(chunk.constant_pool.len(), 5);
    }

    #[test]
    fn test_nested_grouping() {
        let (chunk, success) = compile_expression("((1 + 2) * 3)");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
    }

    #[test]
    fn test_double_negation() {
        let (chunk, success) = compile_expression("--5");
        assert!(success);
        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();
        let negate_count = code_bytes.iter().filter(|&&b| b == OpCode::OpNegate as u8).count();
        assert_eq!(negate_count, 2, "Should have two negation operations");
    }

    #[test]
    fn test_negation_in_expression() {
        let (chunk, success) = compile_expression("-1 + 2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpNegate));
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_decimal_numbers() {
        let (chunk, success) = compile_expression("3.14");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_whitespace_handling() {
        let (chunk, success) = compile_expression("  1   +   2  ");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_error_missing_operand() {
        let (_, success) = compile_expression("1 +");
        assert!(!success, "Should fail with missing operand");
    }

    #[test]
    fn test_error_missing_closing_paren() {
        let (_, success) = compile_expression("(1 + 2");
        assert!(!success, "Should fail with missing closing parenthesis");
    }

    #[test]
    fn test_error_unexpected_character() {
        let (_, success) = compile_expression("1 @ 2");
        assert!(!success, "Should fail with unexpected character");
    }

    #[test]
    fn test_error_empty_expression() {
        let (_, success) = compile_expression("");
        assert!(!success, "Should fail with empty expression");
    }

    #[test]
    fn test_error_only_operator() {
        let (_, success) = compile_expression("+");
        assert!(!success, "Should fail with only operator");
    }

    #[test]
    fn test_error_empty_parens() {
        let (_, success) = compile_expression("()");
        assert!(!success, "Should fail with empty parentheses");
    }

    #[test]
    fn test_left_associativity_addition() {
        let (chunk, success) = compile_expression("1 + 2 + 3");
        assert!(success);
        // Should compile as: 1, 2, +, 3, +
        // This tests left-to-right associativity
        let add_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpAdd as u8).count();
        assert_eq!(add_count, 2);
    }

    #[test]
    fn test_left_associativity_subtraction() {
        let (chunk, success) = compile_expression("10 - 5 - 2");
        assert!(success);
        // Should compile as: 10, 5, -, 2, -
        // Result should be (10 - 5) - 2 = 3, not 10 - (5 - 2) = 7
        let sub_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpSubtract as u8).count();
        assert_eq!(sub_count, 2);
    }

    #[test]
    fn test_mixed_precedence() {
        let (chunk, success) = compile_expression("2 * 3 + 4 * 5");
        assert!(success);
        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();

        // Find positions of operators
        let positions: Vec<(usize, &str)> = code_bytes.iter().enumerate()
            .filter_map(|(i, &b)| {
                match b {
                    b if b == OpCode::OpMultiply as u8 => Some((i, "mul")),
                    b if b == OpCode::OpAdd as u8 => Some((i, "add")),
                    _ => None
                }
            })
            .collect();

        // Both multiplications should come before addition
        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0].1, "mul");
        assert_eq!(positions[1].1, "mul");
        assert_eq!(positions[2].1, "add");
    }

    #[test]
    fn test_complex_nested_expression() {
        let (chunk, success) = compile_expression("((1 + 2) * (3 - 4)) / 5");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpSubtract));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
        assert!(has_opcode(&chunk, OpCode::OpDivide));
    }

    #[test]
    fn test_unary_with_binary() {
        let (chunk, success) = compile_expression("-1 * -2");
        assert!(success);
        let negate_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpNegate as u8).count();
        assert_eq!(negate_count, 2);
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
    }

    #[test]
    fn test_large_number() {
        let (chunk, success) = compile_expression("123456789.987654321");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_zero() {
        let (chunk, success) = compile_expression("0");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_division_before_subtraction() {
        let (chunk, success) = compile_expression("10 - 8 / 2");
        assert!(success);
        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();
        let div_pos = code_bytes.iter().position(|&b| b == OpCode::OpDivide as u8).unwrap();
        let sub_pos = code_bytes.iter().position(|&b| b == OpCode::OpSubtract as u8).unwrap();
        assert!(div_pos < sub_pos, "Division should come before subtraction");
    }

    #[test]
    fn test_multiplication_before_subtraction() {
        let (chunk, success) = compile_expression("10 - 2 * 3");
        assert!(success);
        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();
        let mul_pos = code_bytes.iter().position(|&b| b == OpCode::OpMultiply as u8).unwrap();
        let sub_pos = code_bytes.iter().position(|&b| b == OpCode::OpSubtract as u8).unwrap();
        assert!(mul_pos < sub_pos);
    }

    #[test]
    fn test_same_precedence_left_to_right() {
        let (chunk, success) = compile_expression("10 / 2 * 3");
        assert!(success);
        // Should be: 10, 2, /, 3, *  => (10/2)*3 = 15, not 10/(2*3) = 1.67
        assert!(has_opcode(&chunk, OpCode::OpDivide));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
    }

    // ===== Nested Grouping Tests =====

    #[test]
    fn test_deeply_nested_parens() {
        let (chunk, success) = compile_expression("(((1)))");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_multiple_nested_groups() {
        let (chunk, success) = compile_expression("((1 + 2) * (3 + 4)) + ((5 - 6) / (7 - 8))");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpSubtract));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
        assert!(has_opcode(&chunk, OpCode::OpDivide));
    }

    #[test]
    fn test_nested_negations_with_parens() {
        let (chunk, success) = compile_expression("-(-(1))");
        assert!(success);
        let negate_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpNegate as u8).count();
        assert_eq!(negate_count, 2);
    }

    // ===== Unary Operation Tests =====

    #[test]
    fn test_unary_minus_with_addition() {
        let (chunk, success) = compile_expression("-5 + 10");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpNegate));
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_unary_minus_with_multiplication() {
        let (chunk, success) = compile_expression("-3 * 4");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpNegate));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
    }

    #[test]
    fn test_unary_on_grouped_expression() {
        let (chunk, success) = compile_expression("-(1 + 2)");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpNegate));

        let code_bytes: Vec<u8> = chunk.code.iter().map(|b| b.byte).collect();
        let add_pos = code_bytes.iter().position(|&b| b == OpCode::OpAdd as u8).unwrap();
        let neg_pos = code_bytes.iter().position(|&b| b == OpCode::OpNegate as u8).unwrap();
        assert!(add_pos < neg_pos, "Addition should happen before negation");
    }

    #[test]
    fn test_triple_negation() {
        let (chunk, success) = compile_expression("---7");
        assert!(success);
        let negate_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpNegate as u8).count();
        assert_eq!(negate_count, 3);
    }

    #[test]
    fn test_negation_after_operator() {
        let (chunk, success) = compile_expression("5 * -3");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpNegate));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
    }

    // ===== Number Format Tests =====

    #[test]
    fn test_leading_zero_decimal() {
        let (chunk, success) = compile_expression("0.5");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_trailing_zeros() {
        let (chunk, success) = compile_expression("1.00");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_very_small_number() {
        let (chunk, success) = compile_expression("0.00001");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_multiple_digits() {
        let (chunk, success) = compile_expression("999999");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    // ===== Complex Expression Tests =====

    #[test]
    fn test_all_operations_combined() {
        let (chunk, success) = compile_expression("1 + 2 - 3 * 4 / 5");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
        assert!(has_opcode(&chunk, OpCode::OpSubtract));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
        assert!(has_opcode(&chunk, OpCode::OpDivide));
    }

    #[test]
    fn test_mixed_mul_div() {
        let (chunk, success) = compile_expression("24 / 2 / 3 * 4");
        assert!(success);
        let div_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpDivide as u8).count();
        let mul_count = chunk.code.iter().filter(|b| b.byte == OpCode::OpMultiply as u8).count();
        assert_eq!(div_count, 2);
        assert_eq!(mul_count, 1);
    }

    // ===== Whitespace and Comment Tests =====

    #[test]
    fn test_no_whitespace() {
        let (chunk, success) = compile_expression("1+2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_tabs_and_spaces() {
        let (chunk, success) = compile_expression("\t1\t+\t2\t");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_newlines_in_expression() {
        let (chunk, success) = compile_expression("1\n+\n2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_comment_before_expression() {
        let (chunk, success) = compile_expression("// comment\n1 + 2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_comment_after_expression() {
        let (chunk, success) = compile_expression("1 + 2 // comment");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_comment_in_middle() {
        let (chunk, success) = compile_expression("1 + // comment\n2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    // ===== Error Case Tests =====

    #[test]
    fn test_error_double_operator() {
        let (_, success) = compile_expression("1 ++ 2");
        assert!(!success);
    }

    #[test]
    fn test_error_missing_left_operand() {
        let (_, success) = compile_expression("* 2");
        assert!(!success);
    }

    #[test]
    fn test_error_missing_right_operand() {
        let (_, success) = compile_expression("1 *");
        assert!(!success);
    }

    #[test]
    fn test_error_unmatched_left_paren() {
        let (_, success) = compile_expression("(1 + 2");
        assert!(!success);
    }

    #[test]
    fn test_error_unmatched_right_paren() {
        let (_, success) = compile_expression("1 + 2)");
        assert!(!success);
    }

    #[test]
    fn test_error_empty_parens_in_expression() {
        let (_, success) = compile_expression("1 + () + 2");
        assert!(!success);
    }

    #[test]
    fn test_error_just_operator() {
        let (_, success) = compile_expression("*");
        assert!(!success);
    }

    #[test]
    fn test_error_just_minus() {
        let (_, success) = compile_expression("-");
        assert!(!success);
    }

    #[test]
    fn test_error_division_by_nothing() {
        let (_, success) = compile_expression("10 /");
        assert!(!success);
    }

    #[test]
    fn test_error_multiple_decimals() {
        let (_, success) = compile_expression("1.2.3");
        assert!(!success);
    }

    // ===== Edge Case Tests =====

    #[test]
    fn test_single_zero() {
        let (chunk, success) = compile_expression("0");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_negative_zero() {
        let (chunk, success) = compile_expression("-0");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpNegate));
    }

    #[test]
    fn test_zero_operations() {
        let (chunk, success) = compile_expression("0 + 0");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_many_parentheses() {
        let (chunk, success) = compile_expression("((((((1))))))");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

   
    #[test]
    fn test_expression_with_only_groups() {
        let (chunk, success) = compile_expression("(1)");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 1);
    }

    #[test]
    fn test_nested_unary_in_binary() {
        let (chunk, success) = compile_expression("1 + -2 * 3");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpNegate));
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
        assert!(has_opcode(&chunk, OpCode::OpAdd));
    }

    #[test]
    fn test_large_constant_pool() {
        let (chunk, success) = compile_expression("1+2+3+4+5+6+7+8+9+10");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 10);
    }

    #[test]
    fn test_decimal_in_operation() {
        let (chunk, success) = compile_expression("3.14 * 2");
        assert!(success);
        assert!(has_opcode(&chunk, OpCode::OpMultiply));
        assert_eq!(chunk.constant_pool.len(), 2);
    }

    #[test]
    fn test_all_decimal_operations() {
        let (chunk, success) = compile_expression("1.5 + 2.5 - 0.5 * 2.0 / 0.5");
        assert!(success);
        assert_eq!(chunk.constant_pool.len(), 5);
    }
}
use crate::builtins::math::transformer::TransformStep;

pub fn format_steps(steps: &[TransformStep]) -> Vec<String> {
    steps
        .iter()
        .map(|s| format!("Step {}: {}  # {}", s.step_num, s.expression, s.rule))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::transformer::TransformStep;

    #[test]
    fn test_empty() {
        assert!(format_steps(&[]).is_empty());
    }

    #[test]
    fn test_single_step() {
        let steps = vec![TransformStep {
            step_num: 0,
            expression: "3x - 12 = 0".to_string(),
            rule: "initial expression".to_string(),
        }];
        assert_eq!(format_steps(&steps), vec!["Step 0: 3x - 12 = 0  # initial expression"]);
    }

    #[test]
    fn test_multiple_steps() {
        let steps = vec![
            TransformStep { step_num: 0, expression: "3x - 12 = 0".to_string(), rule: "initial expression".to_string() },
            TransformStep { step_num: 1, expression: "3x = 12".to_string(), rule: "add 12".to_string() },
            TransformStep { step_num: 2, expression: "x = 4".to_string(), rule: "divide by 3".to_string() },
        ];
        let formatted = format_steps(&steps);
        assert_eq!(formatted[0], "Step 0: 3x - 12 = 0  # initial expression");
        assert_eq!(formatted[1], "Step 1: 3x = 12  # add 12");
        assert_eq!(formatted[2], "Step 2: x = 4  # divide by 3");
    }
}

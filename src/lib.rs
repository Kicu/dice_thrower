// enum Die {
//     D4,
//     D6,
//     D8,
//     D10,
//     D12,
//     D20,
//     D100,
// }

#[derive(Debug)]
pub struct DiceThrowExpression {
    die: i32,
    dice_count: usize,
    addend: Option<i32>,
}

impl DiceThrowExpression {
    // Todo better error type
    pub fn build(expr: &str) -> Result<Self, String> {
        let parts_by_space: Vec<&str> = expr.split(" ").collect();

        let (dice_part, addend) = if parts_by_space.len() == 1 {
            (parts_by_space[0], None)
        } else {
            (parts_by_space[0], Some(parts_by_space[1]))
        };

        let parts: Vec<&str> = dice_part.split("d").collect();
        if parts.len() < 2 {
            return Err(format!("unsupported dice expression: {}", expr))
        }

        let die = match parts[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                return Err(format!("unsupported dice expression: {}", expr))
            },
        };

        let dice_count = match parts[0] {
            "" => 1,
            _ => {
                match parts[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(format!("unsupported dice expression: {}", expr))
                    },
                }
            }
        };

        // Todo handle addend
        let addend = None;

        let final_expression = DiceThrowExpression {
            die,
            dice_count,
            addend,
        };

        println!("{:?}", final_expression);

        Ok(final_expression)
    }

    pub fn die(&self) -> i32 {
        self.die
    }

    pub fn die_name(&self) -> String {
        format!("d{}", self.die)
    }

    pub fn dice_count(&self) -> usize {
        self.dice_count
    }
}

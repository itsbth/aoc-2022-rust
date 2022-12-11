#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Value(u64),
}
#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}
#[derive(Debug, Clone, Copy)]
struct Expr(Op, Value, Value);

impl Expr {
    #[inline]
    fn eval(expr: &Self, old: u64) -> u64 {
        match expr {
            Expr(Op::Add, Value::Old, Value::Old) => old + old,
            Expr(Op::Add, Value::Old, Value::Value(v)) => old + v,
            Expr(Op::Add, Value::Value(v), Value::Old) => v + old,
            Expr(Op::Add, Value::Value(v1), Value::Value(v2)) => v1 + v2,
            Expr(Op::Mul, Value::Old, Value::Old) => old * old,
            Expr(Op::Mul, Value::Old, Value::Value(v)) => old * v,
            Expr(Op::Mul, Value::Value(v), Value::Old) => v * old,
            Expr(Op::Mul, Value::Value(v1), Value::Value(v2)) => v1 * v2,
        }
    }
}
pub struct Monkey {
    expr: Expr,
    divisor: u64,
    if_zero: usize,
    if_non_zero: usize,
}

/*
Input example:
Monkey 7:
  Starting items: 70, 60, 71, 69, 77, 70, 98
  Operation: new = old * 7
  Test: divisible by 3
    If true: throw to monkey 2
    If false: throw to monkey 3
*/

pub fn generator(input: &str) -> Vec<(Monkey, Vec<u64>)> {
    use aoc_parse::{parser, prelude::*};

    let operand = parser!({
        (v: u64) => Value::Value(v),
        "old" => Value::Old,
    });

    let operator = parser!({
        "+" => Op::Add,
        "*" => Op::Mul,
    });

    let expression = parser!(
        (lhs: operand) (op: operator) (rhs: operand) => Expr(op, lhs, rhs)
    );

    let monkey = parser!(
            line("Monkey #" usize)
            line(" . Starting items:" (initial: repeat_sep(u64, ", ")))
            line(" . Operation: new = " (expr: expression))
            line(" . Test: divisible by " (divisor: u64))
            line(" .   If true: throw to monkey #" (if_zero: usize))
            line(" .   If false: throw to monkey #" (if_non_zero: usize))
    );
    let data = parser!(sections(monkey)).parse(input).unwrap();
    data.iter()
        .map(|(_id, initial, expr, divisor, if_zero, if_non_zero)| {
            (
                Monkey {
                    expr: *expr,
                    divisor: *divisor,
                    if_zero: *if_zero,
                    if_non_zero: *if_non_zero,
                },
                initial.clone(),
            )
        })
        .collect()
}

pub fn part_1(input: &Vec<(Monkey, Vec<u64>)>) -> usize {
    0
}

use std::vec;

use rayon::prelude::*;

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
        (lhs: operand) " " (op: operator) " " (rhs: operand) => Expr(op, lhs, rhs)
    );

    let monkey = parser!(
            line("Monkey " usize ":")
            line("  Starting items: " (initial: repeat_sep(u64, ", ")))
            line("  Operation: new = " (expr: expression))
            line("  Test: divisible by " (divisor: u64))
            line("    If true: throw to monkey " (if_zero: usize))
            line("    If false: throw to monkey " (if_non_zero: usize))
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
    let mut inventory = input
        .iter()
        .map(|(_, initial)| initial.clone())
        .collect::<Vec<_>>();
    let monkeys = input.iter().map(|(monkey, _)| monkey).collect::<Vec<_>>();
    let mut inspected = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            let inv = inventory[idx].drain(..).collect::<Vec<_>>();
            for item in inv {
                let new = Expr::eval(&monkey.expr, item) / 3;
                inspected[idx] += 1;
                if new % monkey.divisor == 0 {
                    inventory[monkey.if_zero].push(new);
                } else {
                    inventory[monkey.if_non_zero].push(new);
                }
            }
        }
    }
    // multiply the two largest inspected values
    inspected.sort();
    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

pub fn part_2(input: &Vec<(Monkey, Vec<u64>)>) -> usize {
    let mut inventory = input
        .iter()
        .map(|(_, initial)| initial.clone())
        .collect::<Vec<_>>();
    let monkeys = input.iter().map(|(monkey, _)| monkey).collect::<Vec<_>>();
    let mut inspected = vec![0; monkeys.len()];
    let div: u64 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..10_000 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            let inv = inventory[idx].drain(..).collect::<Vec<_>>();
            for item in inv {
                let new = Expr::eval(&monkey.expr, item) % div;
                inspected[idx] += 1;
                if new % monkey.divisor == 0 {
                    inventory[monkey.if_zero].push(new);
                } else {
                    inventory[monkey.if_non_zero].push(new);
                }
            }
        }
    }
    // println!("{:?}", inspected.iter().cloned().sum::<usize>());
    // multiply the two largest inspected values
    inspected.sort();
    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

fn one_round(
    monkeys: &Vec<&Monkey>,
    sv: u64,
    si: usize,
    inspected: &mut Vec<usize>,
    div: u64,
) -> (u64, usize) {
    let mut si = si;
    let mut sv = sv;
    loop {
        let monkey = &monkeys[si];
        sv = Expr::eval(&monkey.expr, sv) % div;
        inspected[si] += 1;
        let next = if sv % monkey.divisor == 0 {
            monkey.if_zero
        } else {
            monkey.if_non_zero
        };
        if next <= si {
            return (sv, next);
        }
        si = next;
    }
}

pub fn part_2_rayon(input: &Vec<(Monkey, Vec<u64>)>) -> usize {
    let monkeys = input.iter().map(|(monkey, _)| monkey).collect::<Vec<_>>();
    let div: u64 = monkeys.iter().map(|m| m.divisor).product();
    let inventory = input.iter().map(|(_, initial)| initial.clone());
    let pairs = inventory
        .enumerate()
        .flat_map(|(i, v)| v.iter().map(|n| (i, *n)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let counts = pairs
        .par_iter()
        .map(|(s, v)| {
            (0..10_000)
                .fold(
                    (*s, *v, vec![0; monkeys.len()]),
                    |(monke, v, mut inspected), _| {
                        let (v, monke) = one_round(&monkeys, v, monke, &mut inspected, div);
                        (monke, v, inspected)
                    },
                )
                .2
        })
        .reduce(
            || vec![0; monkeys.len()],
            |mut a, b| {
                for (i, v) in b.iter().enumerate() {
                    a[i] += v;
                }
                a
            },
        );
    // multiply the two largest inspected values
    let mut counts = counts.iter().collect::<Vec<_>>();
    // println!("{:?}", counts.iter().cloned().sum::<usize>());
    counts.sort();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

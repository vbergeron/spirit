use crate::SpiritValue;

fn numeric_op(
    lhs: SpiritValue,
    rhs: SpiritValue,
    f: fn(i64, i64) -> i64,
) -> Result<SpiritValue, String> {
    match (lhs, rhs) {
        (SpiritValue::Const(l), SpiritValue::Const(r)) => Ok(SpiritValue::Const(f(l, r))),
        (_, _) => Err("type error".to_string()),
    }
}

fn unpack2(mut args: Vec<SpiritValue>) -> Result<(SpiritValue, SpiritValue), String> {
    if args.len() == 2 {
        let arg1 = args.pop().ok_or("err".to_string())?;
        let arg0 = args.pop().ok_or("err".to_string())?;
        Ok((arg0, arg1))
    } else {
        Err(format!("wrong arity: expected {}, got {}", 2, args.len()))
    }
}

pub fn add(args: Vec<SpiritValue>) -> Result<SpiritValue, String> {
    let (lhs, rhs) = unpack2(args)?;
    numeric_op(lhs, rhs, |x, y| x + y)
}

pub fn mul(args: Vec<SpiritValue>) -> Result<SpiritValue, String> {
    let (lhs, rhs) = unpack2(args)?;
    numeric_op(lhs, rhs, |x, y| x * y)
}

pub fn eq(args: Vec<SpiritValue>) -> Result<SpiritValue, String> {
    let (lhs, rhs) = unpack2(args)?;
    numeric_op(lhs, rhs, |x, y| if x == y { 1 } else { 0 })
}

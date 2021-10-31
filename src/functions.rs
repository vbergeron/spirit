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

pub fn add(lhs: SpiritValue, rhs: SpiritValue) -> Result<SpiritValue, String> {
    numeric_op(lhs, rhs, |x, y| x + y)
}

pub fn eq(lhs: SpiritValue, rhs: SpiritValue) -> Result<SpiritValue, String> {
    numeric_op(lhs, rhs, |x, y| if x == y { 1 } else { 0 })
}

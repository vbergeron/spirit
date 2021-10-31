#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpiritValue {
Const(i64),
Lit(String),
Let(String, Box<SpiritValue>, Box<SpiritValue>),
Function(String, Box<SpiritValue>),
Apply(Box<SpiritValue>, Box<SpiritValue>),
Native2(Box<SpiritValue>, Box<SpiritValue>, fn(SpiritValue, SpiritValue) -> SpiritValue),
Cond(Box<SpiritValue>, Box<SpiritValue>, Box<SpiritValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Number
{
    Unsigned(u64),
    Integer(i64),
    Float(f64),
}

impl Number
{
    pub fn to_string(&self) -> String
    {
        match self {
            Number::Unsigned(value) => value.to_string(),
            Number::Integer(value)  => value.to_string(),
            Number::Float(value)    => value.to_string(),
        }
    }
}

impl From<u64> for Number
{
    fn from(number: u64) -> Number
    {
        Number::Unsigned(number)
    }
}

impl From<i64> for Number
{
    fn from(number: i64) -> Number
    {
        Number::Integer(number)
    }
}

impl From<f64> for Number
{
    fn from(number: f64) -> Number
    {
        Number::Float(number)
    }
}

impl From<Number> for u64
{
    fn from(number: Number) -> u64
    {
        match number {
            Number::Unsigned(value) => value,
            Number::Integer(value)  => value as u64,
            Number::Float(value)    => value as u64,
        }
    }
}

impl From<Number> for i64
{
    fn from(number: Number) -> i64
    {
        match number {
            Number::Unsigned(value) => value as i64,
            Number::Integer(value)  => value,
            Number::Float(value)    => value as i64,
        }
    }
}

impl From<Number> for f64
{
    fn from(number: Number) -> f64
    {
        match number {
            Number::Unsigned(value) => value as f64,
            Number::Integer(value)  => value as f64,
            Number::Float(value)    => value,
        }
    }
}


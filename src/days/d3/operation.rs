#[derive(Default)]
pub(crate) struct Operation {
    pub(crate) a: i32,
    pub(crate) b: i32,
}

impl Operation {
    pub(crate) fn compute(&self) -> i32 {
        self.a * self.b
    }
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = value.split(',').collect();
        if tokens.len() == 2 {
            let a = tokens[0].parse::<i32>().map_err(|e| ())?;
            let b = tokens[1].parse::<i32>().map_err(|e| ())?;
            return Ok(Operation { a, b });
        }
        Err(())
    }
}

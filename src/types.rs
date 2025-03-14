pub struct Cols {
    pub values: Vec<usize>,
    pub include: bool,
}

impl Cols {
    pub fn include(values: Vec<usize>) -> Self {
        Self {
            values,
            include: true,
        }
    }
    pub fn skip(values: Vec<usize>) -> Self {
        Self {
            values,
            include: false,
        }
    }
}

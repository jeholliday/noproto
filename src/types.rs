/// Fixed32 integer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed32 {
    /// Value
    pub value: u32,
}

impl Fixed32 {
    /// Create a new Fixed32
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

/// Fixed64 integer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed64 {
    /// Value
    pub value: u64,
}

impl Fixed64 {
    /// Create a new Fixed64
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

/// SFixed32 integer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SFixed32 {
    /// Value
    pub value: i32,
}

impl SFixed32 {
    /// Create a new SFixed32
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

/// SFixed64 integer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SFixed64 {
    /// Value
    pub value: i64,
}

impl SFixed64 {
    /// Create a new SFixed64
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

/// Int32 integer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Int32 {
    /// Value
    pub value: i32,
}

impl Int32 {
    /// Create a new Int32
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

/// Int64 integer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Int64 {
    /// Value
    pub value: i64,
}

impl Int64 {
    /// Create a new Int64
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

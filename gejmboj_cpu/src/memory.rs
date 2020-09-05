use std::fmt::Display;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0xFFFF],
        }
    }

    /// Sets a `u8` value in memory.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xAB;
    ///
    /// memory.set(0, value);
    ///
    /// assert_eq!(value, memory.get(0));
    /// ```
    pub fn set(&mut self, location: usize, value: u8) {
        self.memory[location] = value;
    }

    /// Gets a `u8` value from memory.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xAB;
    ///
    /// memory.set(0, value);
    ///
    /// assert_eq!(value, memory.get(0));
    /// ```
    pub fn get(&self, location: usize) -> u8 {
        self.memory[location]
    }

    /// Gets a `u16` value from memory.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xABCD;
    ///
    /// memory.set_u16(42, value);
    ///
    /// assert_eq!(value, memory.get_u16(42));
    /// ```
    pub fn get_u16(&self, location: usize) -> u16 {
        let lo = self.get(location);
        let hi = self.get(location + 1);

        u16::from_le_bytes([lo, hi])
    }

    /// Sets a `u16` value in memory.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xABCD;
    ///
    /// memory.set_u16(0, value);
    ///
    /// assert_eq!(value, memory.get_u16(0));
    /// ```
    pub fn set_u16(&mut self, location: usize, value: u16) {
        let [lo, hi] = value.to_le_bytes();

        self.set(location, lo);
        self.set(location + 1, hi);
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let columns = 16;
        let bytes_string: String = self
            .memory
            .iter()
            .map(|x| format!("{:02x?}", x))
            .collect::<Vec<String>>()
            .chunks(columns)
            .enumerate()
            .map(|(idx, bytes)| format!("{:03x?} | {} |", idx, bytes.join(" ").replace("00", "--")))
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "
       0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    ,-------------------------------------------------,
{}
    `-------------------------------------------------´",
            bytes_string
        )
    }
}

use arbitrary_int::{u6, Number};

#[derive(Debug, Clone, Copy, Default)]
pub struct ProgramCounter(u6, u6);

impl ProgramCounter {
    pub fn set(&mut self, value: (u6, u6)) {
        self.0 = value.0;
        self.1 = value.1;
    }

    pub fn increment(&mut self) {
        if self.1 < u6::MAX {
            self.1 += u6::new(1);
        } else {
            self.1 = u6::new(0);
            self.0 = self.0.wrapping_add(u6::new(1));
        }
    }

    pub fn as_tuple(&self) -> (u6, u6) {
        (self.0, self.1)
    }
}

#[cfg(test)]
mod test {
    use arbitrary_int::{u6, Number};

    use super::ProgramCounter;

    #[test]
    fn test_pc_as_tuple() {
        let pc = ProgramCounter::default();
        assert_eq!(pc.as_tuple(), (u6::new(0), u6::new(0)));
    }

    #[test]
    fn test_pc_set() {
        let mut pc = ProgramCounter::default();
        assert_eq!(pc.as_tuple(), (u6::new(0), u6::new(0)));

        let new_tuple = (u6::new(5), u6::new(12));
        pc.set(new_tuple);
        assert_eq!(pc.as_tuple(), new_tuple);
    }

    #[test]
    fn test_pc_increment() {
        let mut pc = ProgramCounter::default();

        // Test normal increment
        pc.increment();
        assert_eq!(pc.as_tuple(), (u6::new(0), u6::new(1)));

        // Test increment at max value of second component
        pc.set((u6::new(0), u6::MAX));
        pc.increment();
        assert_eq!(pc.as_tuple(), (u6::new(1), u6::new(0)));

        // Test increment at max value of both components
        pc.set((u6::MAX, u6::MAX));
        pc.increment();
        assert_eq!(pc.as_tuple(), (u6::new(0), u6::new(0)));
    }
}

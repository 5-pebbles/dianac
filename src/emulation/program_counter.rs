use std::{rc::Rc, sync::Mutex};

use arbitrary_int::{u6, Number};

#[derive(Debug, Clone, Default)]
pub struct ProgramCounter(Rc<Mutex<(u6, u6)>>);

impl ProgramCounter {
    pub fn set(&self, value: (u6, u6)) {
        *self.0.lock().unwrap() = value;
    }

    pub fn increment(&mut self) {
        let mut internal = self.0.lock().unwrap();
        if internal.1 < u6::MAX {
            internal.1 += u6::new(1);
        } else {
            internal.1 = u6::new(0);
            internal.0 = internal.0.wrapping_add(u6::new(1));
        }
    }

    pub fn as_tuple(&self) -> (u6, u6) {
        *self.0.lock().unwrap()
    }
}

#[cfg(test)]
mod test {
    use arbitrary_int::{u6, Number};

    use super::ProgramCounter;

    #[test]
    fn test_pc_as_tuple() {
        let pc1 = ProgramCounter::default();

        assert_eq!(pc1.as_tuple(), (u6::new(0), u6::new(0)));
    }

    #[test]
    fn test_pc_set() {
        let pc1 = ProgramCounter::default();
        let pc2 = pc1.clone();

        let default_tuple = (u6::new(0), u6::new(0));
        assert_eq!(pc1.as_tuple(), default_tuple);
        assert_eq!(pc2.as_tuple(), default_tuple);

        let new_tuple = (u6::new(5), u6::new(12));
        pc1.set(new_tuple);
        assert_eq!(pc1.as_tuple(), new_tuple);
        assert_eq!(pc2.as_tuple(), new_tuple);
    }

    #[test]
    fn test_pc_increment() {
        let mut pc1 = ProgramCounter::default();
        let pc2 = pc1.clone();

        // Test normal increment
        pc1.increment();
        assert_eq!(pc1.as_tuple(), (u6::new(0), u6::new(1)));
        assert_eq!(pc2.as_tuple(), (u6::new(0), u6::new(1)));

        // Test increment at max value of second component
        pc1.set((u6::new(0), u6::MAX));
        pc1.increment();
        assert_eq!(pc1.as_tuple(), (u6::new(1), u6::new(0)));
        assert_eq!(pc2.as_tuple(), (u6::new(1), u6::new(0)));

        // Test increment at max value of both components
        pc1.set((u6::MAX, u6::MAX));
        pc1.increment();
        assert_eq!(pc1.as_tuple(), (u6::new(0), u6::new(0)));
        assert_eq!(pc2.as_tuple(), (u6::new(0), u6::new(0)));
    }
}

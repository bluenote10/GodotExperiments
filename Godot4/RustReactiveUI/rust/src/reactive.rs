use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::rc::Rc;

//#[derive(Clone)]
pub struct Dynamic<T>
where
    T: Debug,
{
    value: Rc<RefCell<T>>,
    value_id: Rc<Cell<u64>>,
}

impl<T> Clone for Dynamic<T>
where
    T: Debug,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            value_id: self.value_id.clone(),
        }
    }
}

impl<T> Dynamic<T>
where
    T: Debug,
{
    pub fn new(x: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(x)),
            value_id: Rc::new(Cell::new(0)),
        }
    }

    pub fn update(&self, update: impl FnOnce(&T) -> T) {
        let y = {
            let x = self.value.borrow();
            update(&x)
        };
        self.value.replace(y);
        self.value_id.set(self.value_id.get() + 1);
        println!(
            "Set value to {:?}, value count: {:?}",
            self.value, self.value_id
        );
    }

    pub fn into_consumer(&self) -> Consumer<T> {
        Consumer::new(self.clone())
    }
}

pub struct Consumer<T>
where
    T: Debug,
{
    dynamic: Dynamic<T>,
    consumed_id: Cell<u64>,
}

impl<T> Consumer<T>
where
    T: Debug,
{
    pub fn new(dynamic: Dynamic<T>) -> Self {
        Self {
            dynamic,
            consumed_id: Cell::new(u64::MAX),
        }
    }

    pub fn on_change(&self, f: impl FnOnce(&T)) {
        let value_id = self.dynamic.value_id.get();
        // println!("{} {}", value_id, self.consumed_id.get());
        if value_id != self.consumed_id.get() {
            f(&self.dynamic.value.borrow());
            self.consumed_id.set(value_id);
        }
    }
}

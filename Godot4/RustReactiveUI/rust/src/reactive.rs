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

/*
trait IntoConsumer {
    type Output;
    fn into_consumer(&self) -> Self::Output;
}

impl<A, B> IntoConsumer for (A, B) {
    type Output = (A, B);
    fn into_consumer(&self) -> Self::Output {
        self
    }
}
*/

pub struct Consumer2<A, B>
where
    A: Debug,
    B: Debug,
{
    dynamics: (Dynamic<A>, Dynamic<B>),
    consumed_ids: Cell<(u64, u64)>,
}

impl<A, B> Consumer2<A, B>
where
    A: Debug,
    B: Debug,
{
    pub fn new(dynamics: (Dynamic<A>, Dynamic<B>)) -> Self {
        Self {
            dynamics,
            consumed_ids: Cell::new((u64::MAX, u64::MAX)),
        }
    }

    pub fn on_change(&self, f: impl FnOnce((&A, &B))) {
        let value_id0 = self.dynamics.0.value_id.get();
        let value_id1 = self.dynamics.1.value_id.get();
        // println!("{} {}", value_id, self.consumed_id.get());
        if value_id0 != self.consumed_ids.get().0 || value_id1 != self.consumed_ids.get().1 {
            f((
                &self.dynamics.0.value.borrow(),
                &self.dynamics.1.value.borrow(),
            ));
            self.consumed_ids.set((value_id0, value_id1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_dynamics() {
        let dynamic_a = Dynamic::new(10);
        let dynamic_b = Dynamic::new(20);

        let consumer_a = Consumer::new(dynamic_a.clone());
        let consumer_b = Consumer::new(dynamic_b.clone());
        let consumer_ab = Consumer2::new((dynamic_a.clone(), dynamic_b.clone()));

        let poll = || {
            let mut res_a = None;
            let mut res_b = None;
            let mut res_ab = None;
            consumer_a.on_change(|a| res_a = Some(*a));
            consumer_b.on_change(|b| res_b = Some(*b));
            consumer_ab.on_change(|(a, b)| res_ab = Some((*a, *b)));
            (res_a, res_b, res_ab)
        };

        let (res_a, res_b, res_ab) = poll();
        assert_eq!(res_a, Some(10));
        assert_eq!(res_b, Some(20));
        assert_eq!(res_ab, Some((10, 20)));

        dynamic_a.update(|a| a + 1);

        let (res_a, res_b, res_ab) = poll();
        assert_eq!(res_a, Some(11));
        assert_eq!(res_b, None);
        assert_eq!(res_ab, Some((11, 20)));

        dynamic_b.update(|b| b + 2);

        let (res_a, res_b, res_ab) = poll();
        assert_eq!(res_a, None);
        assert_eq!(res_b, Some(22));
        assert_eq!(res_ab, Some((11, 22)));

        dynamic_a.update(|a| a + 10);
        dynamic_b.update(|b| b + 11);

        let (res_a, res_b, res_ab) = poll();
        assert_eq!(res_a, Some(21));
        assert_eq!(res_b, Some(33));
        assert_eq!(res_ab, Some((21, 33)));
    }
}

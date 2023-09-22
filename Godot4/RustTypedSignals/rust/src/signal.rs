use godot::prelude::*;

pub struct Signal<A>
where
    A: Clone,
{
    callbacks: Vec<Box<dyn Fn(A)>>,
}

impl<A> Signal<A>
where
    A: Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn connect<T>(&mut self, callback: Callback<T, A>)
    where
        T: GodotClass,
    {
        self.callbacks.push(Box::new(move |a: A| callback.call(a)));
    }

    pub fn emit(&self, args: A) {
        for callback in &self.callbacks {
            callback(args.clone());
        }
    }
}

pub struct Callback<T, A>
where
    T: GodotClass,
    A: Clone,
{
    node: Gd<T>,
    func: Box<dyn Fn(Gd<T>, A)>,
}

impl<T, A> Callback<T, A>
where
    T: GodotClass,
    A: Clone + 'static,
{
    pub fn new(node: Gd<T>, func: impl Fn(Gd<T>, A) + 'static) -> Self {
        Callback {
            node,
            func: Box::new(func),
        }
    }

    fn call(&self, a: A) {
        if self.node.is_instance_valid() {
            (self.func)(self.node.share(), a)
        }
    }
}

use godot::prelude::*;

pub struct Signal<A>
where
    A: Clone,
{
    callbacks: Vec<Box<dyn FnMut(A)>>,
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

    pub fn connect<T>(&mut self, mut callback: Callback<T, A>)
    where
        T: GodotClass,
    {
        // let wrapped_callback = |a: A| { callback.func()}

        self.callbacks.push(Box::new(move |a: A| callback.call(a)));
    }

    pub fn emit(&mut self, args: A) {
        for callback in &mut self.callbacks {
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
    func: Box<dyn FnMut(Gd<T>, A)>,
}

impl<T, A> Callback<T, A>
where
    T: GodotClass,
    A: Clone + 'static,
{
    pub fn new(node: Gd<T>, func: impl FnMut(Gd<T>, A) + 'static) -> Self {
        Callback {
            node,
            func: Box::new(func),
        }
    }

    fn call(&mut self, a: A) {
        if self.node.is_instance_valid() {
            (self.func)(self.node.share(), a)
        }
    }
}

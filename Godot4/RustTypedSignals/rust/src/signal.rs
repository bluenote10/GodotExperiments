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
        self.callbacks.push(Box::new(move |args: A| {
            let dispatched_successfully = callback.call(args);
            assert!(dispatched_successfully);
        }));
    }

    pub fn emit(&self, args: A) {
        if let Some((last_callback, other_callbacks)) = self.callbacks.split_last() {
            for callback in other_callbacks {
                // If we have more than one callback we need to clone args...
                callback(args.clone());
            }
            // For last callback can we move, leading to move only in case of 1 receiver.
            last_callback(args);
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

    pub fn with_cast<S>(base: Gd<T>, func: impl Fn(Gd<S>, A) + 'static) -> Self
    where
        S: GodotClass + Inherits<T>,
    {
        Callback::new(base.share(), move |base, arg| {
            // Note that there is no need to check `base.is_instance_valid()` because `call`
            // already checks it on the outside, and doesn't even call the `func` callback
            // if it is invalid.
            let node = base.share().cast::<S>();
            func(node, arg);
        })
    }

    fn call(&self, args: A) -> bool {
        if self.node.is_instance_valid() {
            (self.func)(self.node.share(), args);
            true
        } else {
            false
        }
    }
}

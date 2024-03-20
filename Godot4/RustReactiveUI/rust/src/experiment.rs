trait DoIt1<T> {
    fn do_it(&self, f: impl FnMut(&T));
}

impl<A> DoIt1<A> for (A,) {
    fn do_it(&self, f: impl FnOnce(&A)) {
        f(&self.0);
    }
}

fn test() {
    let tuple = ("a",);
    tuple.do_it(|a| println!("{a}"));
}

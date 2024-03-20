/*
trait DoIt1<A> {
    fn do_it(&self, f: impl FnOnce(&A));
}

impl<A> DoIt1<A> for (A,) {
    fn do_it(&self, f: impl FnOnce(&A)) {
        f(&self.0);
    }
}

trait DoIt2<A, B> {
    fn do_it(&self, f: impl FnOnce((&A, &B)));
}

impl<A, B> DoIt2<A, B> for (A, B) {
    fn do_it(&self, f: impl FnOnce((&A, &B))) {
        f((&self.0, &self.1));
    }
}

trait DoIt3<A, B, C> {
    fn do_it(&self, f: impl FnOnce((&A, &B, &C)));
}

impl<A, B, C> DoIt3<A, B, C> for (A, B, C) {
    fn do_it(&self, f: impl FnOnce((&A, &B, &C))) {
        f((&self.0, &self.1, &self.2));
    }
}

fn main() {
    ("a",).do_it(|x| println!("{x}"));
    ("a", 1).do_it(|(x, y)| println!("{x}, {y}"));
    ("a", 1, true).do_it(|(x, y, z)| println!("{x}, {y}, {z}"));
}
*/

/*
trait DoIt<T> {
    fn do_it(&self, f: impl FnOnce(T));
}

impl<A: Copy> DoIt<(A,)> for (A,) {
    fn do_it(&self, f: impl FnOnce((A,))) {
        f((self.0,));
    }
}
impl<A: Copy, B: Copy> DoIt<(A, B)> for (A, B) {
    fn do_it(&self, f: impl FnOnce((A, B))) {
        f((self.0, self.1));
    }
}
impl<A: Copy, B: Copy, C: Copy> DoIt<(A, B, C)> for (A, B, C) {
    fn do_it(&self, f: impl FnOnce((A, B, C))) {
        f((self.0, self.1, self.2));
    }
}

fn main() {
    ("a",).do_it(|(x,)| println!("{x}"));
    ("a", 1).do_it(|(x, y)| println!("{x}, {y}"));
    ("a", 1, true).do_it(|(x, y, z)| println!("{x}, {y}, {z}"));
}
*/

/*
trait DoIt<T> {
    fn do_it(&self, f: impl FnOnce(T));
}

impl<A> DoIt<&A> for (A,) {
    fn do_it(&self, f: impl FnOnce((&A,))) {
        f((&self.0,));
    }
}
impl<A, B> DoIt<(&A, &B)> for (A, B) {
    fn do_it(&self, f: impl FnOnce((&A, &B))) {
        f((&self.0, &self.1));
    }
}
impl<A, B, C> DoIt<(&A, &B, &C)> for (A, B, C) {
    fn do_it(&self, f: impl FnOnce((&A, &B, &C))) {
        f((&self.0, &self.1, &self.2));
    }
}

fn main() {
    ("a",).do_it(|x| println!("{x}"));
    ("a", 1).do_it(|(x, y)| println!("{x}, {y}"));
    ("a", 1, true).do_it(|(x, y, z)| println!("{x}, {y}, {z}"));
}
*/

trait DoIt<'a, T> {
    fn do_it(&'a self, f: impl FnOnce(T));
}

impl<'a, A> DoIt<'a, (&'a A,)> for (A,) {
    fn do_it(&'a self, f: impl FnOnce((&'a A,))) {
        f((&self.0,))
    }
}

impl<'a, A, B> DoIt<'a, (&'a A, &'a B)> for (A, B) {
    fn do_it(&'a self, f: impl FnOnce((&'a A, &'a B))) {
        f((&self.0, &self.1))
    }
}

impl<'a, A, B, C> DoIt<'a, (&'a A, &'a B, &'a C)> for (A, B, C) {
    fn do_it(&'a self, f: impl FnOnce((&'a A, &'a B, &'a C))) {
        f((&self.0, &self.1, &self.2))
    }
}

fn main() {
    ("a",).do_it(|x| println!("{:?}", x));
    ("a", 1).do_it(|(x, y)| println!("{:?}, {:?}", x, y));
    ("a", 1, true).do_it(|(x, y, z)| println!("{:?}, {:?}, {:?}", x, y, z));
}

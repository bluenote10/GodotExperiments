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

/*
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
*/

// https://stackoverflow.com/questions/78196702/trait-with-a-callback-how-to-model-argument-reference-lifetime/78196812#78196812

use std::cell::RefCell;
use std::rc::Rc;

struct Holder<T>(Rc<RefCell<T>>);

/*
// This works for a single Holder<T>. Note that the generic is A but the FnOnce arg is &A
trait Callback<A> {
    fn callback(&self, f: impl FnOnce(&A));
}

impl<A> Callback<A> for Holder<A> {
    fn callback(&self, f: impl FnOnce(&A)) {
        f(&self.0.borrow())
    }
}
*/

// This tries to move the reference to the generic argument Callback<&A>
/*
144 |     fn callback(&self, f: impl FnOnce(A));
    |     -------------------------------------- definition of `callback` from trait
...
148 |     fn callback(&self, f: impl FnOnce(&A)) {
    |                                ^^^^^^^^^^ impl has extra requirement `for<'a> impl FnOnce(&A): FnOnce(&'a A)`
*/
/*
trait Callback<A> {
    fn callback(&self, f: impl FnOnce(A));
}

impl<A> Callback<&A> for Holder<A> {
    fn callback(&self, f: impl FnOnce(&A)) {
        f(&self.0.borrow())
    }
}

impl<A, B> Callback<(&A, &B)> for (Holder<A>, Holder<B>) {
    fn callback(&self, f: impl FnOnce((&A, &B))) {
        f((&self.0 .0.borrow(), &self.1 .0.borrow()))
    }
}
*/

/*
trait Callback<'a, Args> {
    fn callback(&'a self, f: impl FnOnce(Args));
}

impl<'a, A> Callback<'a, &'a A> for Holder<A> {
    fn callback(&'a self, f: impl FnOnce(&'a A)) {
        f(&self.0.borrow())
    }
}

impl<'a, A, B> Callback<'a, (&'a A, &'a B)> for (Holder<A>, Holder<B>) {
    fn callback(&'a self, f: impl FnOnce((&'a A, &'a B))) {
        f((&self.0 .0.borrow(), &self.1 .0.borrow()))
    }
}
*/

trait Callback<F> {
    fn callback(&self, f: F);
}

impl<A, F> Callback<F> for Holder<A>
where
    F: FnOnce(&A),
{
    fn callback(&self, f: F) {
        f(&self.0.borrow())
    }
}

impl<A, B, F> Callback<F> for (Holder<A>, Holder<B>)
where
    F: FnOnce((&A, &B)),
{
    fn callback(&self, f: F) {
        f((&self.0 .0.borrow(), &self.1 .0.borrow()))
    }
}

fn main() {
    let holder_a = Holder(Rc::new(RefCell::new(42)));
    holder_a.callback(|x| println!("{x}"));

    // Goal
    let holder_a = Holder(Rc::new(RefCell::new(42)));
    let holder_b = Holder(Rc::new(RefCell::new(1.0)));
    (holder_a, holder_b).callback(|(a, b)| println!("{a} {b}"));
}

//! Caching of a single value that can be obtained by evaluating a closure. The closure
//! presumably represents an expensive operation, the result of which never changes.

#![no_std]

/// Lazily evaluated value supplied by a closure.
pub struct Lazy<F, T>(State<F, T>) where F: FnOnce() -> T;

impl<F, T> Lazy<F, T> where F: FnOnce() -> T {
    /// Creates a new [`Lazy`] backed by the given closure.
    pub fn new(f: F) -> Self {
        Self(State::Uninit(Some(f)))
    }

    /// Obtains the memoized value, evaluating the closure if unset.
    pub fn get(&mut self) -> &T {
        if let State::Uninit(f) = &mut self.0 {
            let val = f.take().expect("cannot be uninitialised")();
            self.0 = State::Init(val);
        }

        self.try_get().expect("cannot be uninitialised")
    }

    /// Tries to obtain the memoized value if it is set. The closure is not evaluated
    /// if unset.
    pub fn try_get(&self) -> Option<&T> {
        match &self.0 {
            State::Uninit(_) => None,
            State::Init(val) => Some(val)
        }
    }
}

/// Creates a new [`Lazy`] for a `T` that implements [`Default`].
pub fn default<T: Default>() -> Lazy<impl FnOnce() -> T, T> {
    Lazy::new(|| T::default())
}

enum State<F, T> where F: FnOnce() -> T {
    Uninit(Option<F>),
    Init(T)
}

#[cfg(test)]
mod tests;
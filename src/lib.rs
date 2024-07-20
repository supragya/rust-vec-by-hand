use std::ptr::NonNull;

/// Our implementation of vector is going to be close to std
/// implementation
pub struct MyVec<T> {
    /// A pointer to the first element in the vector. `NonNull`
    /// provides us with a vector that should never be null
    /// unlike what *mut T can do
    ///
    /// This could very well have been `Option<NonNull<T>>` as it makes
    /// it easier to implement `MyVec<T>::new()` as then `ptr` could
    /// have simply be set as `None`.
    ///
    /// `NonNull` is allowed to be "dangling" but it should not be used
    /// without checking.
    ptr: NonNull<T>,

    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            // effectively being set to something random out here.
            // Access needs to be checked
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn std_vec_test() {
    //    let mut vec = std::vec::Vec::new();
    //    vec.push(1usize):
    //    vec.push(2);
    //    vec.push(3);
    //    vec.push(4);
    //    vec.push(5);
    //
    //    assert_eq!(vec.capacity(), 8);
    //    assert_eq!(vec.len(), 5);
    //}

    #[test]
    fn myvec_test() {
        let mut vec = MyVec::<usize>::new();

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}

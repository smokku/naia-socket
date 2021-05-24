cfg_if! {
    if #[cfg(feature = "multithread")] {
        use std::{
            ops::{Deref, DerefMut},
            sync::{Arc, Mutex, MutexGuard},
        };

        #[derive(Debug)]
        pub struct Guard<'a, T: ?Sized> {
            inner: MutexGuard<'a, T>,
        }

        impl<'a, T: ?Sized> Deref for Guard<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<'a, T: ?Sized> DerefMut for Guard<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        /// A reference abstraction that can handle single-threaded and multi-threaded environments
        #[derive(Debug)]
        pub struct Ref<T: ?Sized> {
            inner: Arc<Mutex<T>>,
        }

        impl<T> Ref<T> {
            /// Creates a new 'Ref' containing 'value'
            pub fn new(value: T) -> Self {
                Ref {
                    inner: Arc::new(Mutex::new(value)),
                }
            }
        }

        impl<T: ?Sized> Ref<T> {

            /// Immutably borrows the wrapped value
            pub fn borrow(&self) -> Guard<T> {
                Guard {
                    inner: self.inner.lock().unwrap(),
                }
            }

            /// Mutably borrows the wrapped value
            pub fn borrow_mut(&self) -> Guard<T> {
                Guard {
                    inner: self.inner.lock().unwrap(),
                }
            }

            /// Returns a ref to the inner smart pointer
            pub fn inner(&self) -> Arc<Mutex<T>> {
                return self.inner.clone();
            }

            /// Creates a new 'Ref' containing raw smart pointer 'inner'
            pub fn new_raw(inner: Arc<Mutex<T>>) -> Self {
                Ref {
                    inner,
                }
            }
        }

        impl<T: ?Sized> Clone for Ref<T> {
            fn clone(&self) -> Self {
                Ref {
                    inner: self.inner.clone(),
                }
            }
        }

    } else {
        use std::{
            cell::{Ref as StdRef, RefMut as StdRefMut, RefCell},
            ops::{Deref, DerefMut},
            rc::Rc,
        };

        #[derive(Debug)]
        pub struct Guard<'a, T: ?Sized> {
            inner: StdRef<'a, T>,
        }

        impl<'a, T: ?Sized> Deref for Guard<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                Deref::deref(&self.inner)
            }
        }

        #[derive(Debug)]
        pub struct GuardMut<'a, T: ?Sized> {
            inner: StdRefMut<'a, T>,
        }

        impl<'a, T: ?Sized> Deref for GuardMut<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                Deref::deref(&self.inner)
            }
        }

        impl<'a, T: ?Sized> DerefMut for GuardMut<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        /// A reference abstraction that can handle single-threaded and multi-threaded
        /// environments
        #[derive(Debug)]
        pub struct Ref<T: ?Sized> {
            inner: Rc<RefCell<T>>,
        }

        impl<T> Ref<T> {
            /// Creates a new 'Ref' containing 'value'
            pub fn new(value: T) -> Self {
                Ref {
                    inner: Rc::new(RefCell::new(value)),
                }
            }
        }

        impl<T: ?Sized> Ref<T> {
            /// Immutably borrows the wrapped value
            pub fn borrow(&self) -> Guard<T> {
                Guard {
                    inner: self.inner.borrow(),
                }
            }

            /// Mutably borrows the wrapped value
            pub fn borrow_mut(&self) -> GuardMut<T> {
                GuardMut {
                    inner: self.inner.borrow_mut(),
                }
            }

            /// Returns a ref to the inner smart pointer
            pub fn inner(&self) -> Rc<RefCell<T>> {
                return self.inner.clone();
            }

            /// Creates a new 'Ref' containing raw smart pointer 'inner'
            pub fn new_raw(inner: Rc<RefCell<T>>) -> Self {
                Ref {
                    inner,
                }
            }
        }

        impl<T: ?Sized> Clone for Ref<T> {
            fn clone(&self) -> Self {
                Ref {
                    inner: self.inner.clone(),
                }
            }
        }
    }
}

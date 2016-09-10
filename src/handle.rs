//! Convenient, reference-counted, thread-safe cells.

// Copyright © 2016  John Croisant
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::clone::Clone;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};


/// A convenience wrapper around `Arc<RwLock<T>>`.
///
/// You can clone a Handle to create new Handles referring to the same contents (like an `Arc`).
/// If all Handles to the contents are dropped, the contents will be dropped.
///
/// You must acquire a read or write lock in order to access the Handle's contents (like a `RwLock`).
/// There can be any number of read locks **or** a single write lock at any one point in time.
/// This ensures thread safety when accessing the contents.
///
/// # Examples
///
/// ```
/// # use chipmunk::Handle;
/// // Create a new Handle to some contents, then a mutable clone.
/// let handle1 = Handle::from((2, 3, 4));
/// let mut handle2 = handle1.clone();
///
/// // Temporarily acquire a write lock, and modify the contents.
/// {
///     let mut guard = handle2.write();
///     guard.0 *= guard.0;
///     guard.1 *= guard.1;
///     guard.2 *= guard.2;
/// }
///
/// // Modifying the contents via handle2 also affected handle1.
/// assert_eq!((4, 9, 16), *handle1.read());
/// ```
pub struct Handle<T> {
    inner: Arc<RwLock<T>>
}

impl<T> Handle<T> {
    /// Acquire a read lock, allowing you to read (but not write) the Handle's contents.
    ///
    /// There may be multiple read locks at the same time, but only one write lock.
    /// If there is currently a write lock on these contents,
    /// this function will block the current thread until that lock is released.
    ///
    /// # Panics
    ///
    /// This function panics in situations where `RwLock::read()` would fail,
    /// e.g. if the lock is poisoned.
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read().unwrap()
    }

    /// Acquire a write lock, allowing you to read and/or write the Handle's contents.
    ///
    /// There may be multiple read locks at the same time, but only one write lock.
    /// If there are currently any read locks or a write lock on these contents,
    /// this function will block the current thread until those locks are released.
    ///
    /// # Panics
    ///
    /// This function panics in situations where `RwLock::write()` would fail,
    /// e.g. if the lock is poisoned.
    pub fn write(&mut self) -> RwLockWriteGuard<T> {
        self.inner.write().unwrap()
    }

    /// Create a new WeakHandle which refers to the same contents.
    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle { inner: Arc::downgrade(&self.inner) }
    }
}

impl<T> From<T> for Handle<T> {
    /// Create a new Handle which takes ownership of the contents.
    fn from(contents: T) -> Handle<T> {
        Handle { inner: Arc::new(RwLock::new(contents)) }
    }
}

impl<T> Clone for Handle<T> {
    /// Create a new Handle which refers to the same contents.
    /// This increases the reference count for the lifetime of the new Handle.
    fn clone(&self) -> Handle<T> {
        Handle { inner: self.inner.clone() }
    }
}

impl<T: Debug> Debug for Handle<T> {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        fmt.debug_tuple("Handle")
            .field(self.read().deref())
            .finish()
    }
}


/// A convenience wrapper around `std::sync::Weak<RwLock<T>>`.
///
/// Unlike Handle, WeakHandle does not keep its contents alive.
/// If there are no (strong) Handles to the contents, the contents will be destroyed.
///
/// If you want to access the contents of a WeakHandle, you must upgrade it to a Handle.
/// The upgrade will fail (returning `None`) if the contents no longer exist.
///
/// # Examples
///
/// ```
/// # use chipmunk::{Handle, WeakHandle};
/// // Create a new Handle to some contents.
/// let handle1 = Handle::from((2, 3, 4));
///
/// // Create a new WeakHandle.
/// let weak = &handle1.downgrade();
///
/// // Try to upgrade it so we can access the contents.
/// let mut handle2 = weak.upgrade().unwrap();
///
/// // Temporarily acquire a write lock, and modify the contents.
/// {
///     let mut guard = handle2.write();
///     guard.0 *= guard.0;
///     guard.1 *= guard.1;
///     guard.2 *= guard.2;
/// }
///
/// // Modifying the contents via handle2 also affected handle1.
/// assert_eq!((4, 9, 16), *handle1.read());
/// ```
pub struct WeakHandle<T> {
    inner: Weak<RwLock<T>>
}

impl<T> WeakHandle<T> {
    /// Try to create a new Handle which refers to the same contents.
    /// Returns None if the contents no longer exist.
    pub fn upgrade(&self) -> Option<Handle<T>> {
        match Weak::upgrade(&self.inner) {
            Some(arc) => Some(Handle{ inner: arc }),
            None => None
        }
    }
}

impl<T> Clone for WeakHandle<T> {
    /// Create a shallow clone which refers to the same contents.
    /// This does not affect the reference count.
    fn clone(&self) -> WeakHandle<T> {
        WeakHandle { inner: self.inner.clone() }
    }
}

impl<T: Debug> Debug for WeakHandle<T> {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        fmt.write_str("(WeakHandle)")
    }
}

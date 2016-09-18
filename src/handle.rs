//! Reference-counted interally-mutable cells.

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
use std::cell::{RefCell, Ref, RefMut};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};


/// A reference-counted internally-mutable cell type, based on
/// [`Rc`](https://doc.rust-lang.org/nightly/std/rc/struct.Rc.html)
/// and [`RefCell`](https://doc.rust-lang.org/nightly/std/cell/struct.RefCell.html).
///
/// You can clone a Handle to create new Handles referring to the same contents.
/// If all Handles to the contents are dropped, the contents will be dropped.
///
/// You must borrow the contents in order to access the Handle's contents.
/// There can be any number of immutable borrows **or** a single mutable borrow at any one point in time.
///
/// # Examples
///
/// ```
/// # use chipmunk::Handle;
/// // Create a new Handle to some contents, then a mutable clone.
/// let handle1 = Handle::from((2, 3, 4));
/// let mut handle2 = handle1.clone();
///
/// // Mutably borrow and modify the contents.
/// {
///     let mut contents = handle2.borrow_mut();
///     contents.0 *= contents.0;
///     contents.1 *= contents.1;
///     contents.2 *= contents.2;
/// }
///
/// // Modifying the contents via handle2 also affected handle1.
/// assert_eq!((4, 9, 16), *handle1.borrow());
/// ```
pub struct Handle<T> {
    inner: Rc<RefCell<T>>
}

impl<T> Handle<T> {
    /// Immutably borrows the Handle's contents.
    ///
    /// The borrow lasts until the returned Ref exits scope.
    /// Multiple immutable borrows can be taken out at the same time.
    /// See [`RefCell::borrow`](https://doc.rust-lang.org/nightly/std/cell/struct.RefCell.html#method.borrow).
    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    /// Mutably borrows the Handle's contents.
    ///
    /// The borrow lasts until the returned RefMut exits scope.
    /// The contents cannot be borrowed again (either immutably or mutably) while this borrow is active.
    /// See [`RefCell::borrow_mut`](https://doc.rust-lang.org/nightly/std/cell/struct.RefCell.html#method.borrow_mut).
    pub fn borrow_mut(&mut self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    /// Creates a new WeakHandle which refers to the same contents.
    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle { inner: Rc::downgrade(&self.inner) }
    }
}

impl<T> From<T> for Handle<T> {
    /// Creates a new Handle which takes ownership of the contents.
    fn from(contents: T) -> Handle<T> {
        Handle { inner: Rc::new(RefCell::new(contents)) }
    }
}

impl<T> Clone for Handle<T> {
    /// Creates a new Handle which refers to the same contents.
    fn clone(&self) -> Handle<T> {
        Handle { inner: self.inner.clone() }
    }
}

impl<T: Debug> Debug for Handle<T> {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        fmt.debug_tuple("Handle")
            .field(self.borrow().deref())
            .finish()
    }
}


/// A weak version of Handle, based on
/// [`Weak`](https://doc.rust-lang.org/nightly/std/rc/struct.Weak.html)
/// and [`RefCell`](https://doc.rust-lang.org/nightly/std/cell/struct.RefCell.html).
///
/// Unlike Handle, WeakHandle does not keep its contents alive.
/// If there are no (strong) Handles to the contents, the contents will be dropped.
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
/// let weak = handle1.downgrade();
///
/// // Try to upgrade it so we can access the contents.
/// let mut handle2 = weak.upgrade().unwrap();
///
/// // Mutably borrow and modify the contents.
/// {
///     let mut contents = handle2.borrow_mut();
///     contents.0 *= contents.0;
///     contents.1 *= contents.1;
///     contents.2 *= contents.2;
/// }
///
/// // Modifying the contents via handle2 also affected handle1.
/// assert_eq!((4, 9, 16), *handle1.borrow());
/// ```
pub struct WeakHandle<T> {
    inner: Weak<RefCell<T>>
}

impl<T> WeakHandle<T> {
    /// Tries to create a new Handle which refers to the same contents.
    /// Returns `None` if the contents no longer exist.
    pub fn upgrade(&self) -> Option<Handle<T>> {
        match Weak::upgrade(&self.inner) {
            Some(inner) => Some(Handle{ inner: inner }),
            None => None
        }
    }
}

impl<T> Clone for WeakHandle<T> {
    /// Creates a new WeakHandle which refers to the same contents.
    fn clone(&self) -> WeakHandle<T> {
        WeakHandle { inner: self.inner.clone() }
    }
}

impl<T: Debug> Debug for WeakHandle<T> {
    fn fmt(&self, fmt: &mut Formatter) -> ::std::fmt::Result {
        fmt.write_str("(WeakHandle)")
    }
}

//! `RcList` is read-only, append only list (log), that can share common tail (history) with other `RcList`.
//!
//! This data structure supports:
//!
//! * read-only appending which creates new RcList consisting of:
//!   (head, rest), where head is a new element and rest can be
//!   potentially shared with other `RcList`-s
//! * Strong and Weak links, allowing dynamic pruning,
//! * Iteration from the beginning (newest elements) to end (oldest elements).
//!

use std::rc::{Rc, Weak};
use std::iter::Iterator;
use std::ops::Deref;

#[derive(Clone, Debug)]
enum Link<T> {
    Weak(Weak<T>),
    Strong(Rc<T>),
    None
}

impl<T> Link<T> {
    fn downgrade(&self) -> Link<T> {
        match self {
            &Link::Weak(ref x) => Link::Weak(x.clone()),
            &Link::Strong(ref x) => Link::Weak(Rc::downgrade(x)),
            &Link::None => Link::None
        }
    }
}

#[derive(Clone, Debug)]
/// RcList Node
pub struct Node<T> {
    value: T,
    next: Link<Node<T>>,
}

impl<T> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

/// RcList Head
#[derive(Clone, Debug)]
pub struct RcList<T> {
    first: Link<Node<T>>,
}

impl<T> RcList<T> {

    /// Create new `RcList` with no entries
    pub fn new() -> RcList<T> {
        RcList { first: Link::None }
    }

}

impl<T : Clone> RcList<T> {
    /// Create new `RcList` from value and existing `RcList`
    pub fn new_append(value : T, rest : &RcList<T>) -> RcList<T> {
        let first = rest.first.clone();
        RcList {
            first: Link::Strong(Rc::new(
                           Node {
                               value: value,
                               next: first,
                           }
            ))
        }
    }

    /// Create new `RcList` from value and weakly referenced existing `RcList`
    ///
    /// Weak-reference is useful for limiting memory consumption. After no other `RcList` is
    /// holding a part of the `RcList` with non-weak reference, it will be freed.
    pub fn new_append_weak(value : T, rest : &RcList<T>) -> RcList<T> {
        let first = rest.first.clone();
        RcList {
            first: Link::Strong(Rc::new(
                           Node {
                               value: value,
                               next: first.downgrade(),
                           }
            ))
        }
    }

    /// Get iterator over `RcList`
    pub fn iter(&self) -> RcListIter<T> {
        RcListIter { iter: self.first.clone() }
    }
}

/// Iterator over RcList
pub struct RcListIter<T: 'static> {
    iter : Link<Node<T>>,
}

/// Reference to a `RcList` element
pub struct Ref<T> {
    rc : Rc<Node<T>>
}

impl<T : Clone> Ref<T> {
    pub fn new(rc : Rc<Node<T>>) -> Ref<T> {
        Ref{ rc: rc }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.rc.value
    }
}

impl<T : Clone> Iterator for RcListIter<T> {

    type Item = Ref<T>;

    #[inline]
    fn next(&mut self) -> Option<Ref<T>> {
        let ret = match self.iter {
            Link::None => None,
            Link::Strong(ref rc) => Some(Ref::new(rc.clone())),
            Link::Weak(ref weak) => weak.upgrade().map(|rc| Ref::new(rc)),
        };

        self.iter = match ret {
            Some(ref ret) => ret.rc.next.clone(),
            None => Link::None
        };

        ret
    }
}

#[cfg(test)]
mod test;


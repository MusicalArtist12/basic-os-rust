use core::{
    cell::UnsafeCell, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, Ordering}
};

// writing to a mutex is a critical section - do not interrupt
use crate::{sti, cli};

// token that guarantees mutual exclusion to whatever holds it
pub struct Token<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: *mut T
}

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    lock: AtomicBool,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            data: UnsafeCell::new(data),
            lock: AtomicBool::new(false)
        }
    }

    pub fn lock(&self) -> Token<T> {
        loop {
            if self.lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                cli!();
                return Token {
                    lock: &self.lock,
                    data: self.data.get()
                };
            }
        }
    }

    pub fn force_unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

impl<'a, T: ?Sized> Token<'a, T> {
    pub unsafe fn get_ptr(&mut self) -> *const T {
        self.data
    }
}

impl<'a, T: ?Sized> Drop for Token<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        sti!();
    }
}

impl<'a, T: ?Sized> Deref for Token<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<'a, T: ?Sized> DerefMut for Token<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

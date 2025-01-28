use core::{
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut},
    sync::atomic::{
        AtomicU8,
        Ordering::{Acquire, Relaxed, Release},
    },
};

#[derive(Debug)]
pub struct Mutex<T> {
    locked: AtomicU8,
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Default for Mutex<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Mutex<T> {
    const UNLOCKED: u8 = 0;
    const LOCKED: u8 = 1;

    pub const fn new(init: T) -> Self {
        Self {
            locked: AtomicU8::new(Self::UNLOCKED),
            data: UnsafeCell::new(init),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }

    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            match self.try_lock() {
                Some(guard) => break guard,
                None => (),
            }
            self.spin();
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        if self
            .locked
            .compare_exchange(Self::UNLOCKED, Self::LOCKED, Acquire, Relaxed)
            .is_ok()
        {
            unsafe { Some(MutexGuard::new(self)) }
        } else {
            None
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Relaxed) == Self::LOCKED
    }

    unsafe fn unlock(&self) {
        self.locked.store(Self::UNLOCKED, Release);
    }

    fn spin(&self) {
        while self.is_locked() {
            spin_loop();
        }
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
pub struct MutexGuard<'a, T> {
    parent: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    unsafe fn new(parent: &'a Mutex<T>) -> Self {
        Self { parent }
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.parent.data.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.parent.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.parent.unlock();
        }
    }
}

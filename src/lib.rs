#![no_std]

pub trait Pipeable: Sized{
    #[inline(always)]
    fn pipe<T>(self, fun: impl FnOnce(Self) -> T ) -> T{
        fun(self)
    }
    #[inline(always)]
    fn pipe_apply(mut self, fun: impl FnOnce(&mut Self) ) -> Self{
        fun(&mut self);
        self
    }
}
pub trait RefPipeable{
    #[inline(always)]
    fn pipe_ref<T>(&self, fun: impl FnOnce(&Self) -> T) -> T{
        fun(self)
    }
    #[inline(always)]
    fn pipe_mut<T>(&mut self, fun: impl FnOnce(&mut Self) -> T) -> T{
        fun(self)
    }
}

impl<T: Sized> Pipeable for T{}
impl<T> RefPipeable for T{}

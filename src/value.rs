use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::Deref,
};

use crate::{
    gc::{GarbageCollect, Gc, GcRef},
    obj::{BoundMethod, Class, Closure, Function, Instance, LoxString, NativeFunction},
};

#[derive(Clone, Copy)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    // Following are pointers to garbage collected objects. Value is NOT deep copied.
    String(GcRef<LoxString>),
    Function(GcRef<Function>),
    NativeFunction(GcRef<NativeFunction>),
    Closure(GcRef<Closure>),
    Class(GcRef<Class>),
    Instance(GcRef<Instance>),
    BoundMethod(GcRef<BoundMethod>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(b) => !b,
            Value::Nil => true,
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::NativeFunction(a), Value::NativeFunction(b)) => a == b,
            (Value::Closure(a), Value::Closure(b)) => a == b,
            (Value::Class(a), Value::Class(b)) => a == b,
            (Value::Instance(a), Value::Instance(b)) => a == b,
            (Value::BoundMethod(a), Value::BoundMethod(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(x) => Display::fmt(&x, f),
            Value::Nil => f.write_str("nil"),
            Value::Number(x) => Display::fmt(&x, f),
            Value::String(x) => Display::fmt(x.deref(), f),
            Value::Function(x) => Display::fmt(x.deref(), f),
            Value::NativeFunction(x) => Display::fmt(x.deref(), f),
            Value::Closure(x) => Display::fmt(x.deref(), f),
            Value::Class(x) => Display::fmt(x.deref(), f),
            Value::Instance(x) => Display::fmt(x.deref(), f),
            Value::BoundMethod(x) => Display::fmt(x.deref(), f),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl GarbageCollect for Value {
    fn mark_gray(&mut self, gc: &mut Gc) {
        match self {
            Value::String(x) => x.mark_gray(gc),
            Value::Function(x) => x.mark_gray(gc),
            Value::NativeFunction(x) => x.mark_gray(gc),
            Value::Closure(x) => x.mark_gray(gc),
            _ => {}
        }
    }
}

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A typesafe heterogeneous set
pub struct TypeSet {
    elements: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeSet {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn insert<E: 'static>(&mut self, element: E) {
        self.elements.insert(element.type_id(), Box::new(element));
    }

    pub fn insert_dynamic(&mut self, element: Box<impl Any>) {
        self.elements.insert((*element).type_id(), element);
    }

    pub fn contains<E: 'static>(&self) -> bool {
        self.elements.contains_key(&TypeId::of::<E>())
    }

    pub fn get<E: 'static>(&self) -> Option<&E> {
        self.elements.get(&TypeId::of::<E>())?.downcast_ref()
    }

    pub fn get_mut<E: 'static>(&mut self) -> Option<&mut E> {
        self.elements.get_mut(&TypeId::of::<E>())?.downcast_mut()
    }

    pub fn remove<E: 'static>(&mut self) -> Option<E> {
        Some(*self.elements.remove(&TypeId::of::<E>())?.downcast().ok()?)
    }

    pub fn clear(&mut self) {
        self.elements.clear()
    }
}

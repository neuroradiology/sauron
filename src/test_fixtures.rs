//! This is useful only for testing
//! This is a simple component which just barely comply to being a component
//! use for doing component tests
//!
use crate::html::div;
use crate::Component;
use crate::Program;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SimpleComponent;

impl Component<()> for SimpleComponent {
    fn update(&mut self, _msg: ()) {
        crate::log!("updating in SimpleComponent");
    }
    fn view(&self) -> crate::Node<()> {
        div([], [])
    }
}

pub fn simple_component() -> Rc<RefCell<SimpleComponent>> {
    Rc::new(RefCell::new(SimpleComponent))
}

pub fn simple_program() -> Rc<Program<SimpleComponent, ()>> {
    Program::new_append_to_mount(SimpleComponent, &crate::body())
}

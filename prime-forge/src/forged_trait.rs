use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::EtherealFlow;

use nalgebra_glm as glm;

pub trait ForgedTrait: ForgedHierarchy {
    fn start(&mut self) {}
    fn update(&mut self) {}
}

pub trait ForgedHierarchy: EtherealFlow {
    fn set_father(&mut self, father_id: String);
    fn get_father(&self) -> Option<String>;
}

pub struct TransformSpecialTrait {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    id: uuid::Uuid,
    parent: Option<Weak<RefCell<TransformSpecialTrait>>>,
    children: Vec<Rc<RefCell<TransformSpecialTrait>>>,
    pub model_matrix: glm::Mat4,
}

impl Default for TransformSpecialTrait {
    fn default() -> Self {
        Self {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            children: Vec::new(),
            parent: None,
            model_matrix: glm::Mat4::identity(),
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl TransformSpecialTrait {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_position(mut self, position: glm::Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: glm::Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: glm::Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub(crate) fn set_parent(&mut self, parent: Rc<RefCell<TransformSpecialTrait>>) {
        if let Some(parent) = self.get_parent() {
            parent.borrow_mut().children.retain(|child| child.borrow().id != self.id);
        }
        self.parent = Some(Rc::downgrade(&parent));
    }

    pub(crate) fn set_children(&mut self, children: Vec<Rc<RefCell<TransformSpecialTrait>>>) {
        if children.len() > 0 {
            self.children.extend(children.iter().cloned());
        } else {
            self.children = children;
        }
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<TransformSpecialTrait>>> {
        self.parent.as_ref().and_then(|parent| parent.upgrade())
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<TransformSpecialTrait>>> {
        self.children.clone()
    }

    pub fn get_local_model_matrix(&self) -> glm::Mat4 {
        let transform_x = glm::rotate(
            &glm::Mat4::identity(),
            self.rotation.x.to_radians(),
            &glm::vec3(1.0, 0.0, 0.0),
        );

        let transform_y = glm::rotate(
            &glm::Mat4::identity(),
            self.rotation.y.to_radians(),
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let transform_z = glm::rotate(
            &glm::Mat4::identity(),
            self.rotation.z.to_radians(),
            &glm::vec3(0.0, 0.0, 1.0),
        );

        let rotation = transform_x * transform_y * transform_z;

        let model = glm::Mat4::identity();
        let model = glm::translate(&model, &self.position);
        let model = model * rotation;
        let model = glm::scale(&model, &self.scale);

        model
    }

    pub fn update_self_and_children(&mut self) {
        self.model_matrix = if let Some(parent) = self.get_parent() {
            let parent = parent.borrow();
            parent.get_local_model_matrix() * self.get_local_model_matrix()
        } else {
            self.get_local_model_matrix()
        };

        for child in self.children.iter() {
            child.borrow_mut().update_self_and_children();
        }
    }
}

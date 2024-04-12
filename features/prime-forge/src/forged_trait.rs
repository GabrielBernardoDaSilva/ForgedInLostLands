use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::lost_realm::LostRealm;

use super::EtherealFlow;

use nalgebra_glm as glm;

pub trait ForgedTrait: ForgedHierarchy {
    #[allow(unused_variables)]
    fn start(&mut self, lost_realm: &mut LostRealm) {}
    #[allow(unused_variables)]
    fn update(&mut self, lost_realm: &mut LostRealm, dt: f32) {}
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

    pub fn set_position(&mut self, position: glm::Vec3) {
        self.position = position;
        self.update_self_and_children();
    }

    pub fn set_rotation(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
        self.update_self_and_children();
    }

    pub fn set_scale(&mut self, scale: glm::Vec3) {
        self.scale = scale;
        self.update_self_and_children();
    }

    pub(crate) fn set_hierarchy(
        parent: Rc<RefCell<TransformSpecialTrait>>,
        child: Rc<RefCell<TransformSpecialTrait>>,
    ) {
        if let Some(parent) = child.borrow().parent.as_ref() {
            parent.upgrade().unwrap().borrow_mut().children.retain(|c| {
                c.borrow().id != child.borrow().id
            });
        }
        parent.borrow_mut().children.push(child.clone());
        child.borrow_mut().parent = Some(Rc::downgrade(&parent.clone()));
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
            let parent = unsafe {
                let ptr = parent.as_ptr() as *const TransformSpecialTrait;
                &*ptr
            };
            parent.model_matrix * self.get_local_model_matrix()
        } else {
            self.get_local_model_matrix()
        };
        for child in self.children.iter() {
            child.borrow_mut().update_self_and_children();
        }
    }
}

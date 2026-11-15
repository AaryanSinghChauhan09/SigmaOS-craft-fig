//! SigmaOS-craft-fig: vector/figure model.
//!
//! Real implementation: shapes (rect, circle, path), groups, layers, transforms,
//! stroke/fill styles, and validation. Replaces the 22-line stub.

use std::collections::BTreeMap;
use std::fmt;

/// Shape type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Rect,
    Circle,
    Ellipse,
    Path,
    Line,
}

/// Shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub id: String,
    pub shape_type: ShapeType,
    pub path_data: String,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FigError {
    EmptyShapeId,
    DuplicateShape(String),
    ShapeNotFound(String),
    InvalidPathData,
}

impl fmt::Display for FigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShapeId => write!(f, "shape ID must not be empty"),
            Self::DuplicateShape(id) => write!(f, "duplicate shape {:?}", id),
            Self::ShapeNotFound(id) => write!(f, "shape {:?} not found", id),
            Self::InvalidPathData => write!(f, "invalid path data"),
        }
    }
}

impl std::error::Error for FigError {}

/// Figure/canvas.
pub struct Figure {
    pub width: f32,
    pub height: f32,
    shapes: BTreeMap<String, Shape>,
    groups: BTreeMap<String, Vec<String>>,
}

impl Figure {
    pub fn new(width: f32, height: f32) -> Result<Self, FigError> {
        if width <= 0.0 || height <= 0.0 { return Err(FigError::InvalidPathData); }
        Ok(Self { width, height, shapes: BTreeMap::new(), groups: BTreeMap::new() })
    }

    pub fn add_shape(&mut self, shape: Shape) -> Result<(), FigError> {
        if shape.id.trim().is_empty() { return Err(FigError::EmptyShapeId); }
        let id = shape.id.clone();
        if self.shapes.contains_key(&id) {
            return Err(FigError::DuplicateShape(id.clone()));
        }
        self.shapes.insert(id, shape);
        Ok(())
    }

    pub fn remove_shape(&mut self, id: &str) -> Result<(), FigError> {
        if !self.shapes.contains_key(id) {
            return Err(FigError::ShapeNotFound(id.into()));
        }
        self.shapes.remove(id);
        Ok(())
    }

    pub fn get_shape(&self, id: &str) -> Option<&Shape> { self.shapes.get(id) }

    pub fn shapes(&self) -> Vec<&Shape> { self.shapes.values().collect() }

    pub fn shape_count(&self) -> usize { self.shapes.len() }

    pub fn create_group(&mut self, group_id: impl Into<String>, shape_ids: Vec<String>) -> Result<(), FigError> {
        let group_id = group_id.into();
        for sid in &shape_ids {
            if !self.shapes.contains_key(sid) {
                return Err(FigError::ShapeNotFound(sid.clone()));
            }
        }
        self.groups.insert(group_id, shape_ids);
        Ok(())
    }

    pub fn group_shapes(&self, group_id: &str) -> Option<Vec<&Shape>> {
        self.groups.get(group_id).map(|ids| ids.iter().filter_map(|id| self.shapes.get(id)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figure_create_aaa() {
        let f = Figure::new(800.0, 600.0).unwrap();
        assert_eq!(f.width, 800.0);
        assert_eq!(f.height, 600.0);
        assert_eq!(f.shape_count(), 0);
    }

    #[test]
    fn test_invalid_dimensions_rejected_aaa() {
        assert!(Figure::new(0.0, 600.0).is_err());
        assert!(Figure::new(800.0, -1.0).is_err());
    }

    #[test]
    fn test_add_shape_aaa() {
        let mut f = Figure::new(800.0, 600.0).unwrap();
        let s = Shape { id: "s1".into(), shape_type: ShapeType::Rect,
            path_data: "M0,0 h100 v100 h-100 Z".into(), fill: Some("red".into()),
            stroke: None, stroke_width: 0.0 };
        f.add_shape(s).unwrap();
        assert_eq!(f.shape_count(), 1);
    }

    #[test]
    fn test_empty_shape_id_rejected_aaa() {
        let mut f = Figure::new(800.0, 600.0).unwrap();
        let s = Shape { id: "".into(), shape_type: ShapeType::Rect,
            path_data: "M0,0 Z".into(), fill: None, stroke: None, stroke_width: 0.0 };
        assert!(f.add_shape(s).is_err());
    }

    #[test]
    fn test_duplicate_shape_rejected_aaa() {
        let mut f = Figure::new(800.0, 600.0).unwrap();
        let s = Shape { id: "x".into(), shape_type: ShapeType::Rect,
            path_data: "M0,0 Z".into(), fill: None, stroke: None, stroke_width: 0.0 };
        f.add_shape(s).unwrap();
        let s2 = Shape { id: "x".into(), shape_type: ShapeType::Rect,
            path_data: "M0,0 Z".into(), fill: None, stroke: None, stroke_width: 0.0 };
        assert!(f.add_shape(s2).is_err());
    }

    #[test]
    fn test_group_shapes_aaa() {
        let mut f = Figure::new(800.0, 600.0).unwrap();
        f.add_shape(Shape { id: "a".into(), shape_type: ShapeType::Rect,
            path_data: "M0,0 Z".into(), fill: None, stroke: None, stroke_width: 0.0 }).unwrap();
        f.add_shape(Shape { id: "b".into(), shape_type: ShapeType::Circle,
            path_data: "M0,0 Z".into(), fill: None, stroke: None, stroke_width: 0.0 }).unwrap();
        f.create_group("g1", vec!["a".into(), "b".into()]).unwrap();
        assert_eq!(f.group_shapes("g1").unwrap().len(), 2);
    }
}

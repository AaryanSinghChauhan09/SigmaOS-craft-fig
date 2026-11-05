#[derive(Debug, Clone)]
pub struct FigureElement { pub id: String, pub d: String }

#[derive(Debug, Default)]
pub struct Figure { elements: Vec<FigureElement> }
impl Figure {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, id: impl Into<String>, d: impl Into<String>) {
        self.elements.push(FigureElement { id: id.into(), d: d.into() });
    }
    pub fn count(&self) -> usize { self.elements.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_fig() {
        let mut f = Figure::new();
        f.add("path1", "M0,0 L10,10 Z");
        assert_eq!(f.count(), 1);
    }
}

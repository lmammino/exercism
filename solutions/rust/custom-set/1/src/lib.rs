#[derive(Debug)]
pub struct CustomSet<T: PartialEq + Eq + Clone> {
    pub data: Vec<T>,
}

impl<T: PartialEq + Eq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.len() == other.data.len() && self.is_subset(other)
    }
}

impl<T: PartialEq + Eq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = Self { data: Vec::new() };
        for element in input {
            set.add(element.clone());
        }

        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.iter().all(|element| !other.contains(element))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let new_data: Vec<T> = self
            .data
            .iter()
            .filter(|element| self.contains(element) && other.contains(element))
            .cloned()
            .collect();
        Self { data: new_data }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let new_data: Vec<T> = self
            .data
            .iter()
            .filter(|element| !other.contains(element))
            .cloned()
            .collect();
        Self { data: new_data }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut new_data: Vec<T> = vec![];
        for element in self.data.iter() {
            if !new_data.contains(element) {
                new_data.push(element.clone());
            }
        }
        for element in other.data.iter() {
            if !new_data.contains(element) {
                new_data.push(element.clone());
            }
        }
        Self { data: new_data }
    }
}
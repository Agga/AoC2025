
    use std::fmt;

    use crate::Vec2;

    pub struct Grid<T> {
        pub width: i32,
        pub height: i32,
        pub data: Vec<(Vec2, T)>,
    }

    impl<T> Grid<T> {
        pub fn from_file<F: Fn(char) -> T>(input: &str, func: F) -> Self {
            let width = input.lines().next().unwrap().chars().count() as i32;
            let height = input.lines().count() as i32;

            let data: Vec<(Vec2, T)> = input
                .lines()
                .flat_map(|line| line.chars().map(|c| func(c)))
                .enumerate()
                .map(|(index, value)| {
                    let x = index as i32 % width;
                    let y: i32 = index as i32 / width;
                    (Vec2::new(x, y), value)
                })
                .collect();

            Grid {
                width,
                height,
                data,
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = &(Vec2, T)> {
            self.data.iter()
        }

        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Vec2, T)> {
            self.data.iter_mut()
        }

        // pub fn iter_mut(&mut self) -> impl Iterator<Item = (Vec2, &mut T)> {
        //     self.data.iter_mut()
        // }

        pub fn position_from_index(&self, index: usize) -> Option<Vec2> {
            if index < self.data.len() {
                let x = index as i32 % self.width;
                let y = index as i32 / self.width;
                Some(Vec2::new(x, y))
            } else {
                None
            }
        }

        pub fn value_for(&self, pos: &Vec2) -> Option<&T> {
            if let Some(index) = self.index_for(pos) {
                return Some(&self.data[index].1);
            }
            None
        }

        pub fn index_for(&self, pos: &Vec2) -> Option<usize> {
            if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
                let index = pos.y * self.width + pos.x;
                return Some(index as usize);
            }
            None
        }

        pub fn set_value_for(&mut self, pos: &Vec2, value: T ) {
            if let Some(index) = self.index_for(pos) {
                self.data[index].1 = value;
            }
        }

        // pub fn iter_mut(&mut self) -> GridIter<std::slice::IterMut<'_, T>>
        // {
        //
        // }

        // pub fn new(width: i32, height: i32, data: Vec<T>) -> Self {
        //     Grid {
        //         width,
        //         height,
        //         data,
        //     }
        // }
        //

        //
        // pub fn contains(&self, pos: &Vec2) -> bool {
        //     self.index_for(pos).is_some()
        // }
        //

        //
        // pub fn value_for_checked(&self, pos: &Vec2) -> &T {
        //     if let Some(index) = self.index_for(pos) {
        //         return &self.data[index];
        //     }
        //     panic!("check position first via contains");
        // }
        //

    }

    impl<T> std::fmt::Debug for Grid<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let n = self.width as usize;
            writeln!(f, "Grid {{ w: {} h: {} }}", self.width, self.height).expect("success");
            for (i, item) in self.data.iter().enumerate() {
                write!(f, "{:?}", item).expect("success");
                if (i + 1) % n == 0 {
                    writeln!(f).expect("success");
                }
            }
            Ok(())
        }
    }
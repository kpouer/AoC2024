use crate::days::d2::direction::Direction;
use crate::days::d2::direction::Direction::Increasing;

#[derive(Debug)]
pub struct Report {
    list: Vec<i32>,
}

impl From<Vec<i32>> for Report {
    fn from(list: Vec<i32>) -> Self {
        Report { list }
    }
}

impl Report {
    pub(crate) fn is_valid(&self) -> bool {
        let increasing = self.list[1] > self.list[0];
        self.list[0..self.list.len() - 1]
            .iter()
            .zip(self.list[1..].iter())
            .all(|(x, y)| ((x < y && increasing) || (x > y && !increasing)) && is_in_range(*x, *y))
    }

    fn get_direction(&self) -> Direction {
        let mut increasing = 0;
        let mut decreasing = 0;
        self.list[0..self.list.len() - 1]
            .iter()
            .zip(self.list[1..].iter())
            .map(|(x, y)| x < y)
            .for_each(|b| if b { increasing += 1 } else { decreasing += 1 });
        if increasing <= 1 {
            Direction::Decreasing
        } else if decreasing <= 1 {
            Direction::Increasing
        } else {
            Direction::Indeterminate
        }
    }

    pub(crate) fn is_valid2(&self) -> bool {
        let valid = self.is_valid();
        if valid {
            return true;
        }
        let direction = self.get_direction();
        if direction == Direction::Indeterminate {
            false
        } else {
            self.check_clean(direction == Increasing)
        }
    }

    fn check_clean(&self, increasing: bool) -> bool {
        for index in 0..self.list.len() - 1 {
            let x = *self.list.get(index).unwrap();
            let y = *self.list.get(index + 1).unwrap();
            if !is_ok(x, y, increasing) {
                return self.check_with_removed_index(index)
                    || self.check_with_removed_index(index + 1);
            }
        }
        false
    }

    fn check_with_removed_index(&self, index: usize) -> bool {
        let mut new_list = self.list.clone();
        new_list.remove(index);
        let clean_report = Self { list: new_list };
        clean_report.is_valid()
    }
}

fn is_ok(x: i32, y: i32, increasing: bool) -> bool {
    is_good_direction(x, y, increasing) && is_in_range(x, y)
}

fn is_good_direction(x: i32, y: i32, increasing: bool) -> bool {
    if increasing {
        x < y
    } else {
        x > y
    }
}

fn is_in_range(v1: i32, v2: i32) -> bool {
    (v1 - v2).abs() <= 3 && (v1 - v2).abs() > 0
}

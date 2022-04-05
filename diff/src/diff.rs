use std::collections::VecDeque;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Edit<'a, T> 
    where T : Clone + PartialEq {
    Nil(&'a T),
    Deletion(&'a T),
    Insertion(&'a T),
}

#[derive(Clone, Debug)]
pub struct Path<'a, T> 
    where T : Clone + PartialEq {
    pub sequence : Vec<Edit<'a, T>>,
    endpoint : (usize, usize),
}

impl<'a, T> fmt::Display for Path<'a, T>
    where T : Clone + PartialEq + fmt::Display {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::new();

        for edit in &self.sequence {
            display_string.push_str(&match edit {
                Edit::Nil(item) => format!("  {}\n", item),
                Edit::Deletion(item) => format!("\x1b[31m- {}\x1b[0m\n", item),
                Edit::Insertion(item) => format!("\x1b[32m+ {}\x1b[0m\n", item),
            });
        }

        write!(f, "{}", display_string)
    }
}

impl<'a, T> Path<'a, T> 
    where T : Clone + PartialEq {
    pub fn new(sequence : Vec<Edit<T>>, endpoint : (usize, usize)) -> Path<T> {
        Path {
            sequence,
            endpoint,
        }
    }

    pub fn move_right(self, first : &'a Vec<T>) -> Result<Path<T>, Path<T>> {
        if self.endpoint.0 >= first.len() {
            Err(self)
        }
        else {
            let mut new_sequence = self.sequence.clone();
            new_sequence.push(Edit::Deletion(&first[self.endpoint.0]));

            Ok(
                Path::new(
                    new_sequence,
                    (self.endpoint.0 + 1, self.endpoint.1)
                )
            )
        }
    }

    pub fn move_down(self, second : &'a Vec<T>) -> Result<Path<T>, Path<T>> {
        if self.endpoint.1 >= second.len() {
            Err(self)
        }
        else {
            let mut new_sequence = self.sequence.clone();
            new_sequence.push(Edit::Insertion(&second[self.endpoint.1]));

            Ok(
                Path::new(
                    new_sequence,
                    (self.endpoint.0, self.endpoint.1 + 1)
                )
            )
        }
    }

    pub fn move_diagonal(self, first : &'a Vec<T>, second : &Vec<T>, comparison : fn (&T, &T) -> bool) -> Result<Path<'a, T>, Path<'a, T>> {
        let strings_not_equal =
            first.get(self.endpoint.0) == None
            || second.get(self.endpoint.1) == None
            || !comparison(first.get(self.endpoint.0).unwrap(), second.get(self.endpoint.1).unwrap());

        if strings_not_equal {
            Err(self)
        }
        else {
            let mut new_sequence = self.sequence.clone();
            new_sequence.push(Edit::Nil(&first[self.endpoint.0]));

            Ok(
                Path::new(
                    new_sequence,
                    (self.endpoint.0 + 1, self.endpoint.1 + 1)
                )
            )
        }
    }
}

pub fn diff<'a, T>(first : &'a Vec<T>, second : &'a Vec<T>, comparison : fn (&T, &T) -> bool) -> Path<'a, T>
    where T : Clone + PartialEq + std::fmt::Debug {

    let mut paths_queue = VecDeque::new();
    let mut endpoints = HashSet::new();

    paths_queue.push_back(Path::new(Vec::new(), (0, 0)));
    endpoints.insert((0,0));

    let result = loop {
        let mut curr = paths_queue.pop_front().unwrap();

        curr = loop {
            match curr.move_diagonal(first, second, comparison) {
                Ok(new) => {
                    curr = new;
                    continue;
                },
                Err(new) => break new,
            }
        };

        if !endpoints.contains(&(curr.endpoint.0 + 1, curr.endpoint.1)) {
            if let Ok(new) = curr.clone().move_right(first) {
                endpoints.insert(new.endpoint);
                paths_queue.push_back(new);
            }
        }

        if !endpoints.contains(&(curr.endpoint.0, curr.endpoint.1 + 1)) {
            if let Ok(new) = curr.clone().move_down(second) {
                endpoints.insert(new.endpoint);
                paths_queue.push_back(new);
            }
        }

        if curr.endpoint == (first.len(), second.len()) {
            break curr;
        }
    };

    result
}

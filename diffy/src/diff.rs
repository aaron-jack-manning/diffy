use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::fmt;
use std::cmp;
use std::mem;

#[derive(Clone, Debug)]
pub enum Edit<'a, T> 
    where T : Clone + PartialEq {
    Nil(&'a T),
    Deletion(&'a T),
    Insertion(&'a T),
}

#[derive(Clone, Debug)]
pub struct Path<'a, T> 
    where T : Clone + cmp::PartialEq {
    pub sequence : Vec<Edit<'a, T>>,
    endpoint : (usize, usize),
    deletions : u32,
    insertions : u32,
    consumptions : u32,
}



impl<'a, T> cmp::Ord for Path<'a, T>
    where T : Clone + cmp::PartialEq {
    fn cmp(&self, other : &Self) -> cmp::Ordering {
        other.absolute_depth().cmp(&other.absolute_depth())
    }
}
impl<'a, T> PartialEq for Path<'a, T>
    where T : Clone + cmp::PartialEq {
    fn eq(&self, other : &Self) -> bool {
        self.absolute_depth() == other.absolute_depth()
    }

    fn ne(&self, other : &Self) -> bool {
        self.absolute_depth() != other.absolute_depth()
    }
}
impl<'a, T> cmp::Eq for Path<'a, T>
    where T : Clone + cmp::PartialEq {}
impl<'a, T> cmp::PartialOrd for Path<'a, T>
    where T : Clone + cmp::PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
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

    pub fn start() -> Path<'a, T> {
        Path {
            sequence : Vec::new(),
            endpoint : (0, 0),
            deletions : 0,
            insertions : 0,
            consumptions : 0,
        }
    }

    pub fn new(sequence : Vec<Edit<T>>, endpoint : (usize, usize), deletions : u32, insertions : u32, consumptions : u32) -> Path<T> {
        Path {
            sequence,
            endpoint,
            deletions,
            insertions,
            consumptions,
        }
    }

    pub fn deletion(self, first : &'a Vec<T>) -> Result<Path<T>, Path<T>> {
        if self.endpoint.0 >= first.len() {
            Err(self)
        }
        else {
            let mut new_sequence = self.sequence.clone();
            new_sequence.push(Edit::Deletion(&first[self.endpoint.0]));

            Ok(
                Path::new(
                    new_sequence,
                    (self.endpoint.0 + 1, self.endpoint.1),
                    self.deletions + 1,
                    self.insertions,
                    self.consumptions
                )
            )
        }
    }

    pub fn insertion(self, second : &'a Vec<T>) -> Result<Path<T>, Path<T>> {
        if self.endpoint.1 >= second.len() {
            Err(self)
        }
        else {
            let mut new_sequence = self.sequence.clone();
            new_sequence.push(Edit::Insertion(&second[self.endpoint.1]));

            Ok(
                Path::new(
                    new_sequence,
                    (self.endpoint.0, self.endpoint.1 + 1),
                    self.deletions,
                    self.insertions + 1,
                    self.consumptions
                )
            )
        }
    }

    pub fn consume_free(self, first : &'a Vec<T>, second : &Vec<T>, comparison : fn (&T, &T) -> bool) -> Result<Path<'a, T>, Path<'a, T>> {
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
                    (self.endpoint.0 + 1, self.endpoint.1 + 1),
                    self.deletions,
                    self.insertions,
                    self.consumptions + 1
                )
            )
        }
    }

    pub fn consume_all_free(mut self, first : &'a Vec<T>, second : &'a Vec<T>, comparison : fn (&T, &T) -> bool) -> Path<'a, T> {
        loop {
            match self.consume_free(first, second, comparison) {
                Ok(new) => {
                    self = new;
                    continue;
                },
                Err(new) => break new,
            }
        }
    }

    pub fn absolute_depth(&self) -> i64 {
        i64::from(self.deletions) + i64::from(self.insertions) + 2 * i64::from(self.consumptions)
    }

    pub fn axis(&self) -> i64 {
        i64::try_from(self.deletions).unwrap() - i64::try_from(self.insertions).unwrap()
    }
}

pub fn diff<'a, T>(first : &'a Vec<T>, second : &'a Vec<T>, comparison : fn (&T, &T) -> bool) -> Path<'a, T>
    where T : Clone + PartialEq + std::fmt::Debug + std::fmt::Display { //display here is purely for debugging

    let mut old_queue = BinaryHeap::new();
    let mut new_queue = BinaryHeap::new();
    let mut endpoints = HashSet::new();

    let start = Path::start().consume_all_free(first, second, comparison);
    endpoints.insert(start.endpoint);
    old_queue.push(start);


    let mut axis_best_path = HashMap::new();
    for deletion in 0..(first.len() + 1) {
        for insertion in 0..(second.len() + 1) {
            axis_best_path.insert(i64::try_from(deletion).unwrap() - i64::try_from(insertion).unwrap(), 0);
        } 
    }


    for _depth in 0..(first.len() + second.len()) {

        while !old_queue.is_empty() {

            let curr = old_queue.pop().unwrap();

            if curr.endpoint == (first.len(), second.len()) {
                return curr;
            }

            if curr.absolute_depth() < axis_best_path[&curr.axis()] {
                continue;
            }
            

            if !endpoints.contains(&(curr.endpoint.0 + 1, curr.endpoint.1)) {
                if let Ok(mut new) = curr.clone().deletion(first) {
                    endpoints.insert(new.endpoint);
                    new = new.consume_all_free(first, second, comparison);
                    if new.absolute_depth() > axis_best_path[&new.axis()] {
                        *axis_best_path.get_mut(&new.axis()).unwrap() = new.absolute_depth();
                    }
                    new_queue.push(new);
                }
            }

            if !endpoints.contains(&(curr.endpoint.0, curr.endpoint.1 + 1)) {
                if let Ok(mut new) = curr.clone().insertion(second) {
                    endpoints.insert(new.endpoint);
                    new = new.consume_all_free(first, second, comparison);
                    if new.absolute_depth() > axis_best_path[&new.axis()] {
                        *axis_best_path.get_mut(&new.axis()).unwrap() = new.absolute_depth();
                    }
                    new_queue.push(new);
                }
            }
        }

        mem::swap(&mut old_queue, &mut new_queue);
    }

    panic!("Implementation Error");
}

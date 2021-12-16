type ID = usize;

#[derive(Clone, Debug, PartialEq)]
enum CaveType {
    Small,
    Big,
}


#[derive(Debug)]
struct Caves {
    caves: Vec<(String, CaveType)>,
}

impl Caves {
    fn find(&self, input: &str) -> Option<ID> {
        self.caves.iter().position(|c| c.0.as_str() == input)
    }

    fn copy(&self, id: ID) -> (String, CaveType) {
        (
            self.caves[id].0.clone(),
            self.caves[id].1.to_owned(),
        )
    }

    fn get(&self, id: ID) -> Option<(String, CaveType)> {
        if id < self.caves.len() {
            Some(self.copy(id))
        } else {
            None
        }
    }

    fn push(&mut self, input: &str) -> usize {
        match self.find(input) {
            None => {
                self.caves.push((
                    String::from(input),
                    if input.chars().all(|c| c.is_uppercase()) {
                        CaveType::Big
                    } else {
                        CaveType::Small
                    },
                ));
                self.caves.len() - 1
            }
            Some(id) => id,
        }
    }

    fn new() -> Self {
        Self { caves: Vec::new() }
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.caves.len()
    }
}
struct Connections {
    data: Vec<(ID, Vec<ID>)>,
}

impl Clone for Connections {
    fn clone(&self) -> Self {
        let mut res: Vec<(ID, Vec<ID>)> = Vec::new();
        for (x, y) in &self.data {
            res.push((x.clone(), y.iter().cloned().collect::<Vec<ID>>()));
        }
        Self { data: res }
    }
}

#[derive(Debug)]
struct CavesConnection {
    caves: Caves,
    connections: Vec<(ID, Vec<ID>)>,
}
impl CavesConnection {
    fn new() -> Self {
        Self {
            caves: Caves::new(),
            connections: Vec::new(),
        }
    }

    fn traverse(
        &self,
        current_id: usize,
        current_path: Vec<usize>,
        double_possible: bool,
        used_double: bool,
    ) -> Vec<Vec<usize>> {
        let mut res = Vec::new();
        let to_test = self
            .connections
            .clone()
            .into_iter()
            .filter(|c| c.0 == current_id)
            .map(|c| c.1)
            .collect::<Vec<_>>()[0]
            .clone();

        let end_id = &self.caves.find("end").unwrap();
        let start_id = &self.caves.find("start").unwrap();
        for id in to_test {
            if &id == end_id {
                let mut path = current_path.clone();
                path.push(id);
                res.push(path);
            } else {
                if !current_path.contains(&id) || self.caves.get(id).unwrap().1 == CaveType::Big {
                    let mut current_path_updated = current_path.clone();
                    current_path_updated.push(id);
                    let tmp = self.traverse(id, current_path_updated, double_possible, used_double);
                    for path in tmp {
                        res.push(path);
                    }
                } else if double_possible && !used_double {
                    if current_path.iter().filter(|p| *p == &id).count() == 1
                        && id != *end_id
                        && id != *start_id
                    {
                        let mut current_path_updated = current_path.clone();
                        current_path_updated.push(id);
                        let tmp = self.traverse(id, current_path_updated, double_possible, true);
                        for path in tmp {
                            res.push(path);
                        }
                    }
                }
            }
        }
        res
    }

    fn push(&mut self, input: &str) {
        let (begin, end) = input.split_once('-').unwrap();
        let begin_id = self.caves.push(begin);
        let end_id = self.caves.push(end);
        match self.connections.iter().position(|c| c.0 == begin_id) {
            Some(position) => {
                if !self.connections[position].1.iter().any(|e| e == &end_id) {
                    self.connections[position].1.push(end_id);
                }
            }
            None => {
                self.connections.push((begin_id, vec![end_id]));
            }
        }
        match self.connections.iter().position(|c| c.0 == end_id) {
            Some(position) => {
                if !self.connections[position].1.iter().any(|e| e == &begin_id) {
                    self.connections[position].1.push(begin_id);
                }
            }
            None => {
                self.connections.push((end_id, vec![begin_id]));
            }
        }
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.connections.len()
    }
}

fn process1(input: &str) -> usize {
    let mut con = CavesConnection::new();
    for line in input.lines() {
        con.push(line);
    }
    println!("{:?}", con.caves);
    let start_id = con.caves.find("start").unwrap();
    let output = con.traverse(start_id, vec![start_id], false, false);
    output.len()
}

fn process2(input: &str) -> usize {
    let mut con = CavesConnection::new();
    for line in input.lines() {
        con.push(line);
    }
    println!("{:?}", con.caves);
    let start_id = con.caves.find("start").unwrap();
    let output = con.traverse(start_id, vec![start_id], true, false);
    output.len()
}
fn main() {
    println!(
        "Part 1: {}",
        process1(&std::fs::read_to_string("input").expect("Couldn't read the input"))
    );
    println!(
        "Part 2: {}",
        process2(&std::fs::read_to_string("input").expect("Couldn't read the input"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_caves() {
        let mut a = Caves::new();
        a.push("start");
        assert_eq!(1, a.len());
        a.push("start");
        assert_eq!(1, a.len());
        a.push("end");
        assert_eq!(2, a.len());
    }

    #[test]
    fn test_finding_caves() {
        let mut a = Caves::new();
        a.push("start");
        assert_eq!(1, a.len());
        assert_eq!(Some(0), a.find("start"));
        assert_eq!(None, a.find("end"));
        a.push("end");
        assert_eq!(2, a.len());
        assert_eq!(Some(1), a.find("end"));
    }

    #[test]
    fn test_adding_connection() {
        let mut a = CavesConnection::new();
        a.push("start-A");
        println!("{:?}", a);
        assert_eq!(2, a.len());
        a.push("start-b");
        println!("{:?}", a);
        assert_eq!(3, a.len());
    }

    #[test]
    fn part1_1() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        assert_eq!(10, process1(input));
    }
    #[test]
    fn part1_2() {
        let input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
        assert_eq!(19, process1(input));
    }
    #[test]
    fn part2_1() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        assert_eq!(36, process2(input));
    }
    #[test]
    fn part2_2() {
        let input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
        assert_eq!(103, process2(input));
    }
}

type ID = usize;
#[derive(Clone, Debug)]
enum CaveType {
    Small,
    Big,
}
#[derive(Clone, Debug)]
enum Status {
    Seen,
    NotSeen,
}
#[derive(Debug)]
struct Caves {
    caves: Vec<(String, CaveType, Status)>,
}
impl Caves {
    fn find(&self, input: &str) -> Option<ID> {
        self.caves.iter().position(|c| c.0.as_str() == input)
    }

    fn copy(&self, id: ID) -> (String, CaveType, Status) {
        (
            self.caves[id].0.clone(),
            self.caves[id].1.to_owned(),
            self.caves[id].2.to_owned(),
        )
    }

    fn get(&self, id: ID) -> Option<(String, CaveType, Status)> {
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
                    Status::NotSeen,
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
#[derive(Debug)]
struct CavesConnection {
    caves: Caves,
    connections: Vec<(ID, Vec<ID>)>,
    answers: Vec<Vec<usize>>,
}
impl CavesConnection {
    fn new() -> Self {
        Self {
            caves: Caves::new(),
            connections: Vec::new(),
            answers: Vec::new(),
        }
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
    // let answers = Vec::new();
    println!("{:?}", con.caves);
    let start_id = con.caves.find("start").unwrap();
    let mut to_test = con
        .connections
        .into_iter()
        .filter(|c| c.0 == start_id)
        .map(|c| c.1)
        .collect::<Vec<_>>();
    loop
    {

        if to_test.len() == 0 {
            break;
        }
    }
    0
}
fn main() {
    println!(
        "Part 1: {}",
        process1(&std::fs::read_to_string("input").expect("Couldn't read the input"))
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
}

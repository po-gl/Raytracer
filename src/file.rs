/// # file
/// `file` is a module for I/O

use std::fs::File;
use std::io::{prelude::*};

pub fn write_to_file(str: String, path: String) {
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(str.as_bytes()).expect("Unable to write to file");
    f.sync_all().expect("Unable to sync file");
}



/// # obj_loader
/// `obj_loader` is a module for reading in a Waveform OBJ file
pub mod obj_loader {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use crate::tuple::{Tuple, point};
    use std::ops::{IndexMut, Index};
    use crate::shape::group::Group;
    use crate::shape::Shape;
    use crate::shape::triangle::Triangle;

    /// A one based array
    #[derive(Debug)]
    pub struct OneVec<T> {
       vector: Vec<T>
    }
    impl<T> OneVec<T> {
        pub fn new(vector: Vec<T>) -> OneVec<T> {
            OneVec {vector}
        }

        pub fn push(&mut self, val: T) {
            self.vector.push(val);
        }

        pub fn len(&self) -> usize {
            self.vector.len()
        }
    }

    impl<T> Index<usize> for OneVec<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.vector[index-1]
        }
    }

    impl<T> IndexMut<usize> for OneVec<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.vector[index-1]
        }
    }


    pub struct Parser {
        pub ignored_lines: i32,
        pub vertices: OneVec<Tuple>,
        pub default_group: Group,
    }

    impl Parser {
        pub fn parse_obj_file(path: &str) -> io::Result<(Parser)> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
            let mut parser = Parser {
                ignored_lines: 0,
                vertices: OneVec::new(vec![]),
                default_group: Group::new(),
            };

            let mut progress = 0;
            let total_lines = lines.len();

            for line in lines {
                println!("Progress for obj: {}/{} lines", &progress, total_lines);
                progress += 1;

                let char_res = line.chars().next();
                if char_res.is_none() {
                    continue;
                }
                match char_res.unwrap() {
                    'v' => parser.parse_vertex(&line),
                    'f' => parser.parse_face(&line),
                    _ => parser.ignored_lines += 1
                }
            }
            Ok(parser)
        }

        fn parse_vertex(&mut self, line: &String) {
            let mut vertex = [0.0f64; 3];
            let mut num_counter = 0;
            let mut str_builder = String::from("");
            for character in line.chars() {
                if num_counter > 3 {
                    break;
                }
                match character {
                    'v' => continue,
                    ' '|'\t'|'\n'|'\r'|'\u{2029}' => {
                        let result = Parser::parse_float(&str_builder);
                        if result.is_none() {
                            continue;
                        } else {
                            vertex[num_counter] = result.unwrap();
                            str_builder.clear();
                        }
                    },
                    _ => {
                        str_builder.push(character);
                        continue;
                    },
                }
                num_counter += 1;
            }

            self.vertices.push(point(vertex[0], vertex[1], vertex[2]))
        }

        fn parse_face(&mut self, line: &String) {
            let mut verts: Vec<usize> = vec![];
            let mut str_builder = String::from("");
            let mut should_skip_number: bool = false;


            for character in line.chars() {
                match character {
                    'f' => continue,
                    ' '|'\t'|'\n'|'\r'|'\u{2029}' => {
                        if should_skip_number {
                            str_builder.clear();
                            continue;
                        }
                        let result = Parser::parse_int(&str_builder);
                        if result.is_none() {
                            continue;
                        } else {
                            verts.push(result.unwrap() as usize);
                            str_builder.clear();
                        }
                    },
                    '/' => {
                        should_skip_number = true;
                        let result = Parser::parse_int(&str_builder);
                        if result.is_none() {
                            continue;
                        } else {
                            verts.push(result.unwrap() as usize);
                            str_builder.clear();
                        }
                    }
                    _ => {
                        str_builder.push(character);
                        continue;
                    },
                }
            }

            // Try to parse on end of line
            let result = Parser::parse_int(&str_builder);
            if !result.is_none() {
                verts.push(result.unwrap() as usize);
            }

            if verts.len() >= 3 {
                let mut polygon: OneVec<Tuple> = OneVec::new(vec![]);
                for i in 0..verts.len() {
                    polygon.push(self.vertices[verts[i]])
                }
                let triangles = Parser::fan_triangulations(polygon);
                for tri in triangles {
                    self.default_group.add_child(&mut tri.clone());
                }
            }
        }

        fn parse_float(num_str: &String) -> Option<f64>{
            let result = num_str.parse::<f64>();
            if result.is_err() {
                return None
            } else {
                return Some(result.unwrap());
            }
        }

        fn parse_int(num_str: &String) -> Option<i32>{
            let result = num_str.parse::<i32>();
            if result.is_err() {
                return None
            } else {
                return Some(result.unwrap());
            }
        }

        fn fan_triangulations(vertices: OneVec<Tuple>) -> Vec<Box<dyn Shape>> {
            let mut triangles: Vec<Box<dyn Shape>> = vec![];

            for i in 2..vertices.len() {
                let triangle: Box<dyn Shape> = Box::new(Triangle::new(vertices[1], vertices[i], vertices[i+1]));
                triangles.push(triangle);
            }
            triangles
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn file_obj_parse_ignore() {
            let parser = Parser::parse_obj_file("Obj/gibberish.obj");
            assert_eq!(parser.unwrap().ignored_lines, 5);
        }

        #[test]
        fn file_obj_parse_vertex() {
            let parser = Parser::parse_obj_file("Obj/vertex.obj");
            let uparser = parser.unwrap();
            assert_eq!(uparser.vertices[1], point(-1.0, 1.0, 0.0))
        }

        #[test]
        fn file_obj_parse_faces() {
            let parser = Parser::parse_obj_file("Obj/faces.obj");
            let uparser = parser.unwrap();
            assert_eq!(uparser.vertices[1], point(-1.0, 1.0, 0.0));
            let g = uparser.default_group;
            let t1b = g.shapes[0].clone();
            let t2b = g.shapes[1].clone();

            let t1 = t1b.as_any().downcast_ref::<Triangle>().unwrap();
            let t2 = t2b.as_any().downcast_ref::<Triangle>().unwrap();

            assert_eq!(t1.p1, uparser.vertices[1]);
            assert_eq!(t1.p2, uparser.vertices[2]);
            assert_eq!(t1.p3, uparser.vertices[3]);
            assert_eq!(t2.p1, uparser.vertices[1]);
            assert_eq!(t2.p2, uparser.vertices[3]);
            assert_eq!(t2.p3, uparser.vertices[4]);
        }

        #[test]
        fn file_obj_parse_polygon() {
            let parser = Parser::parse_obj_file("Obj/polygon.txt");
            let uparser = parser.unwrap();
            assert_eq!(uparser.vertices[1], point(-1.0, 1.0, 0.0));
            let g = uparser.default_group;
            let t1b = g.shapes[0].clone();
            let t2b = g.shapes[1].clone();
            let t3b = g.shapes[2].clone();

            let t1 = t1b.as_any().downcast_ref::<Triangle>().unwrap();
            let t2 = t2b.as_any().downcast_ref::<Triangle>().unwrap();
            let t3 = t3b.as_any().downcast_ref::<Triangle>().unwrap();

            assert_eq!(t1.p1, uparser.vertices[1]);
            assert_eq!(t1.p2, uparser.vertices[2]);
            assert_eq!(t1.p3, uparser.vertices[3]);
            assert_eq!(t2.p1, uparser.vertices[1]);
            assert_eq!(t2.p2, uparser.vertices[3]);
            assert_eq!(t2.p3, uparser.vertices[4]);
            assert_eq!(t3.p1, uparser.vertices[1]);
            assert_eq!(t3.p2, uparser.vertices[4]);
            assert_eq!(t3.p3, uparser.vertices[5]);
        }
    }
}



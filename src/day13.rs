use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day13_test1.txt");

        assert_eq!(result, "The sum is 405");
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day13_test1.txt");

        assert_eq!(result, "The sum is 400");
    }
}

pub fn part1(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let mut total: usize = 0;
    
    while let Some(line) = lines.next(){
        let mut diagram: Vec<Vec<Tile>> = Vec::new();
        // lines
        //     .map(
        //         | s | {
        //             s.chars()
        //             .map(| ch | Tile::read(ch))
        //             .collect::<Vec<Tile>>()
        //         }
        //     ).collect();

        let mut sym_points: Option<Vec<usize>> = None;
        let mut working_line = line;
        while working_line != "" {
            let row: Vec<Tile> = working_line.chars().map(|ch| Tile::read(ch)).collect();
            if let Some(prev_points) = sym_points {
                let new_points = confirm_symmetry(&row, &prev_points);
                sym_points = Some(new_points);
            } else {
                let points = find_symmetry(&row);
                sym_points = Some(points);
            }
            // println!("{sym_points:?}");
            diagram.push(row);
            working_line = lines.next().expect("fine prematurely ended");
        }

        let sym_points = sym_points.unwrap();

        match sym_points.len() {
            1 => {total += sym_points[0];},
            0 => {
                let v_sym = find_symmetry(&diagram);
                if v_sym.len() != 1 {
                    panic!("Could not find one horizontal symmetry line");
                }
                total += 100 * v_sym[0];
            },
            _ => panic!("Found more than one vertical symmetry line")
        }
        // println!("{total}");
    }

    format!("The sum is {total}")
}

pub fn part2(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let mut total: usize = 0;
    while let Some(line) = lines.next(){
        let mut diagram: Vec<Vec<Tile>> = Vec::new();

        let mut sym_points: Option<Vec<(usize, bool)>> = None;
        let mut working_line = line;
        while working_line != "" {
            let row: Vec<Tile> = working_line.chars().map(|ch| Tile::read(ch)).collect();
            if let Some(prev_points) = sym_points {
                let new_points = confirm_h_symmetry_fuzz1(&row, &prev_points);
                sym_points = Some(new_points);
            } else {
                let points = find_h_symmetry_fuzz1(&row);
                sym_points = Some(points);
            }
            // println!("{sym_points:?}");
            diagram.push(row);
            working_line = lines.next().expect("fine prematurely ended");
        }

        let fuzzy_sym_points: Vec<usize> = sym_points
                                                            .unwrap()
                                                            .iter()
                                                            .filter(| (_, fuzzy) | *fuzzy)
                                                            .map(| (point, _) | *point)
                                                            .collect();

        match fuzzy_sym_points.len() {
            1 => {total += fuzzy_sym_points[0];},
            0 => {
                let v_sym = find_v_symmetry_fuzz1(&diagram);
                total += 100 * v_sym;
            },
            _ => panic!("Found more than one line of horizontal symmetry")
        }
        // println!("{total}");
    }


    format!("The sum is {total}")
}

fn find_symmetry<T: Eq>(seq: &Vec<T>) -> Vec<usize> {
    let mut sym_points: Vec<usize> = Vec::new();

    for i in 1..seq.len() {
        let symmetric = 
            seq[..i].iter()
                .rev()
                .zip(seq[i..].iter())
                .all(| (l, r) | *l == *r);
        if symmetric {
            sym_points.push(i);
        }
    }

    sym_points
}

fn confirm_symmetry<T: Eq>(seq: &Vec<T>, candidates: &Vec<usize>) -> Vec<usize> {
    let mut sym_points: Vec<usize> = Vec::new();

    for i in candidates {
        let symmetric = 
            seq[..*i].iter()
                .rev()
                .zip(seq[*i..].iter())
                .all(| (l, r) | *l == *r);
        if symmetric {
            sym_points.push(*i);
        }
    }

    sym_points
}

/// Finds indices to the right of vertical lines of symmetry
/// With up to 1 mismatched tiles
/// Boolean part of tuple indicates whether match is fuzzy
fn find_h_symmetry_fuzz1(seq: &Vec<Tile>) -> Vec<(usize, bool)> {
    let mut sym_points: Vec<(usize, bool)> = Vec::new();

    for i in 1..seq.len() {
        let errors = 
            seq[..i].iter().rev()
                .zip(seq[i..].iter())
                .filter(| (l, r) | *l != *r)
                .count();
        if errors <= 1 {
            sym_points.push((i, errors==1));
        }
    }

    sym_points
}

fn confirm_h_symmetry_fuzz1(seq: &Vec<Tile>, candidates: &Vec<(usize, bool)>) -> Vec<(usize, bool)> {
    let mut sym_points: Vec<(usize, bool)> = Vec::new();

    for (i, was_fuzzy) in candidates {
        let errors = 
            seq[..*i].iter().rev()
                .zip(seq[*i..].iter())
                .filter(| (l, r) | *l != *r)
                .count();
        if *was_fuzzy && errors == 0 {
            sym_points.push((*i, true));
        } else if !*was_fuzzy && errors <= 1 {
            sym_points.push((*i, errors==1));
        }
    }

    sym_points
}

fn find_v_symmetry_fuzz1(diagram: &Vec<Vec<Tile>>) -> usize {
    // for each possible horizontal line
    // move outwards counting errors, reject when above 1 or if at 0 at end
    // stop when a reflection point is found
    let n_rows = diagram.len();

    for i in 1..n_rows {
        let rows_to_check = i.min(n_rows-i);
        let mut error_count: usize = 0;
        for j in 0..rows_to_check {
            let row1 = &diagram[i-1-j];
            let row2 = &diagram[i+j];
            error_count += row1.iter()
                            .zip(row2.iter())
                            .filter(| (l, r) | **l != **r)
                            .count();
            if error_count > 1 {
                break;
            }
        }
        if error_count == 1 {
            return i;
        }
    }
    
    panic!("Couldn't find line of vertical symmetry");
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    P, // for '#'
    D, // for '.'
}

impl Tile {
    fn read(ch: char) -> Self {
        match ch {
            '.' => Self::P,
            '#' => Self::D,
            _ => panic!("Invalid character")
        }
    }

    // fn is_P(&self) -> bool {
    //     match self {
    //         Self::P => true,
    //         Self::D => false
    //     }
    // }
    
    // fn is_D(&self) -> bool {
    //     match self {
    //         Self::D => true,
    //         Self::P => false
    //     }
    // }
}

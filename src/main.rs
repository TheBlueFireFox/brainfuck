use std::io::Read;

#[allow(dead_code)]
const PROGRAM_HALLO_WORLD: &str = include_str!("../input.txt");

#[allow(dead_code)]
const PROGRAM_ADDING: &str = include_str!("../input1.txt");

fn main() {
    let int = Interpreter::new(PROGRAM_HALLO_WORLD.as_bytes());
    for _ in int {}
}

struct Interpreter<'program> {
    prog_ptr: usize,
    program: &'program [u8],
    data_ptr: usize,
    cells: Vec<u8>,
    last: Vec<usize>,
}

impl<'program> Interpreter<'program> {
    const SIZE: usize = 30_000;

    fn new<'input, T>(program: &'input T) -> Self
    where
        T: AsRef<[u8]> + 'input + ?Sized,
        'input: 'program,
    {
        Self {
            program: program.as_ref(),
            cells: vec![0; Self::SIZE],
            prog_ptr: 0,
            data_ptr: 0,
            last: Vec::with_capacity(10),
        }
    }
}

impl std::iter::Iterator for Interpreter<'_> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.prog_ptr >= self.program.len() {
            return None;
        }

        // get next
        let curr = self.program[self.prog_ptr];
        match curr {
            b'>' => self.data_ptr += 1,
            b'<' => self.data_ptr -= 1,
            b'+' => self.cells[self.data_ptr] += 1,
            b'-' => self.cells[self.data_ptr] -= 1,
            b'.' => print!("{}", char::from_u32(self.cells[self.data_ptr] as _)?),
            b',' => self.cells[self.data_ptr] = std::io::stdin().bytes().next()?.ok()?,
            b'[' => {
                // find next ]
                let mut idx = 1;
                let mut next = self.prog_ptr;
                while idx > 0 {
                    next += 1;
                    idx += match self.program[next] {
                        b']' => -1,
                        b'[' => 1,
                        _ => 0,
                    };
                }
                self.last.push(self.prog_ptr);

                // jump to ]
                self.prog_ptr = next - 1;
            }
            b']' => {
                if self.cells[self.data_ptr] != 0 {
                    // jump to [
                    let &ptr = self.last.last()?;
                    self.prog_ptr = ptr;
                } else {
                    let _ = self.last.pop();
                }
            }
            _ => {}
        }

        self.prog_ptr += 1;

        Some(())
    }
}

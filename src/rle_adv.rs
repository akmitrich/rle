use std::io::{self, Read, Write};

const BUF_CAP: usize = 1024;

pub fn encode<R, W>(mut input: R, mut output: W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    const MAX_RUN: usize = 127;
    fn write_byte_stretch<W>(output: &mut W, bytes: &[u8]) -> io::Result<()>
    where
        W: Write,
    {
        let count = -(bytes.len() as isize) as i8;
        let count = [count as u8];
        output.write(&count)?;
        output.write(bytes)?;
        Ok(())
    }

    enum State {
        Wait,
        Single(Vec<u8>),
        Run(u8, u8),
    }

    use State::*;
    let mut buf = [0_u8; BUF_CAP];
    let mut state = Wait;
    while let Ok(size) = input.read(&mut buf) {
        if size == 0 {
            break;
        }
        assert!(size <= BUF_CAP);
        for byte in &buf[..size] {
            state = match state {
                Wait => Single(vec![*byte]),
                Single(stretch) if stretch.len() >= MAX_RUN => {
                    write_byte_stretch(&mut output, &stretch)?;
                    Single(vec![*byte])
                }
                Single(mut stretch) if byte == stretch.last().unwrap() => {
                    stretch.pop();
                    if !stretch.is_empty() {
                        write_byte_stretch(&mut output, &stretch)?;
                    }
                    Run(*byte, 2)
                }
                Single(mut stretch) => {
                    stretch.push(*byte);
                    Single(stretch)
                }
                Run(current, count) if current == *byte && (count as usize) < MAX_RUN => {
                    Run(current, count + 1)
                }
                Run(current, count) => {
                    let out = vec![count, current];
                    output.write(&out)?;
                    Single(vec![*byte])
                }
            }
        }
    }
    match state {
        Wait => {}
        Single(stretch) => write_byte_stretch(&mut output, &stretch)?,
        Run(byte, count) => {
            let out = [count, byte];
            output.write(&out)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trivial_test() {
        let input: Vec<u8> = vec![];
        let mut output: Vec<u8> = vec![];
        encode(input.as_slice(), &mut output).unwrap();
        assert_eq!(Vec::<u8>::new(), output);
    }

    #[test]
    fn same_bytes_encode() {
        let input = vec![4_u8, 4, 4, 4];
        let mut output = vec![];
        encode(input.as_slice(), &mut output).unwrap();
        assert_eq!(vec![4_u8, 4], output);
    }

    #[test]
    fn different_bytes_encode() {
        let input = vec![0_u8, 1, 11, 13, 42, 101];
        let mut output = vec![];
        encode(input.as_slice(), &mut output).unwrap();
        assert_eq!(vec![250, 0_u8, 1, 11, 13, 42, 101], output);
    }

    #[test]
    fn some_bytes_encode() {
        let input = vec![0_u8, 1, 1, 1, 42, 42, 101];
        let mut output = vec![];
        encode(input.as_slice(), &mut output).unwrap();
        assert_eq!(vec![255, 0_u8, 3, 1, 2, 42, 255, 101], output);
    }
}

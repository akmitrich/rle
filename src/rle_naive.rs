use std::io::{self, Read, Write};

const BUF_CAP: usize = 1024;

struct Run(u8, u8);

pub fn encode(input: &[u8]) -> Vec<u8> {
    fn write_output(output: &mut Vec<u8>, byte: u8, run: u8) {
        output.push(run);
        output.push(byte);
    }
    if input.is_empty() {
        return vec![];
    }
    let mut output = vec![];
    let mut run = Run(input[0], 1);
    for current in &input[1..] {
        run = match run {
            Run(running, count) if count < u8::MAX && *current == running => {
                Run(running, count + 1)
            }
            Run(running, count) => {
                write_output(&mut output, running, count);
                Run(*current, 1)
            }
        }
    }
    let Run(byte, count) = run;
    write_output(&mut output, byte, count);
    output
}

pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    for two_bytes in input.chunks(2) {
        let (count, byte) = (two_bytes.first().unwrap(), two_bytes.last().unwrap());
        output.extend([*byte].repeat(*count as _));
    }
    output
}

pub fn pack<R, W>(mut input: R, mut output: W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    enum State {
        Wait,
        Run(u8, u8),
    }
    use State::*;

    let mut buf = [0_u8; BUF_CAP];
    let mut state = Wait;
    while let Ok(size) = input.read(&mut buf) {
        if size == 0 {
            // Due to Read trait 0 means end-of-file
            break;
        }
        assert!(size <= BUF_CAP);
        for byte in &buf[..size] {
            state = match state {
                Wait => Run(*byte, 1),
                Run(current, count) if *byte == current && count < u8::MAX => {
                    Run(current, count + 1)
                }
                Run(current, count) => {
                    let out = [count, current];
                    output.write(&out)?;
                    Run(*byte, 1)
                }
            }
        }
    }
    if let Run(byte, count) = state {
        let out = [count, byte];
        output.write(&out)?;
    }
    Ok(())
}

pub fn unpack<R, W>(mut input: R, mut output: W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    enum State {
        Wait,
        Count(u8),
        Byte,
    }
    use State::*;

    let mut buf = [0_u8; BUF_CAP];
    let mut out = [0_u8; u8::MAX as _];
    let mut state = Wait;
    while let Ok(size) = input.read(&mut buf) {
        if size == 0 {
            // Due to Read trait 0 means end-of-file
            break;
        }
        assert!(size <= BUF_CAP);
        for byte in &buf[..size] {
            state = match state {
                Wait => Count(*byte),
                Count(count) => {
                    let buf = &mut out[..count as _];
                    buf.fill(*byte);
                    output.write(buf)?;
                    Byte
                }
                Byte => Count(*byte),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trivial() {
        assert_eq!(Vec::<u8>::new(), encode(&[]));
        assert_eq!(Vec::<u8>::new(), decode(&[]))
    }

    #[test]
    fn zip_n_bytes() {
        assert_eq!(vec![4, 4], encode(&[4, 4, 4, 4]));
    }

    #[test]
    fn compress() {
        assert_eq!(
            vec![2, 4, 1, 11, 3, 0, 1, 21],
            encode(&[4, 4, 11, 0, 0, 0, 21])
        )
    }

    #[test]
    fn wiki_rle_example() {
        let example = b"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";
        let compressed = [
            12, b'W', 1, b'B', 12, b'W', 3, b'B', 24, b'W', 1, b'B', 14, b'W',
        ]; //12'W'1'B'12'W',3,'B',24,'W',1,'B',14,'W'
        assert_eq!(compressed, encode(example).as_slice());
        assert_eq!(example, decode(&compressed).as_slice())
    }

    #[test]
    fn encode_decode() {
        let origin = b"Hello, World!";
        let encoded = encode(origin);
        let decoded = decode(&encoded);
        assert_eq!(origin, decoded.as_slice());
    }

    #[test]
    fn from_compressed() {
        assert_eq!(vec![42, 4, 4, 4, 4, 4, 101], decode(&[1, 42, 5, 4, 1, 101]));
    }
}

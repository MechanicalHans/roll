use fcla::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng as _};
use std::io::{self, Write as _};

fn main() -> fcla::MainResult<io::Error> {
    #[derive(FromArgs)]
    struct Args {
        raw: Option<Box<str>>,
        seed: Option<[u8; 32]>,
    }

    let Args { raw, seed } = fcla::parse_cla::<Args>()?.args;
    let mut rng = match seed {
        Some(seed) => StdRng::from_seed(seed),
        None => StdRng::from_entropy(),
    };
    match raw {
        Some(raw) => cla(&raw, &mut rng)?,
        None => file(io::stdin().lock(), &mut rng)?,
    }
    Ok(())
}

fn cla(raw: &str, rng: &mut impl Rng) -> io::Result<()> {
    eval(raw, rng, io::stdout())
}

fn file(source: impl io::BufRead, rng: &mut impl Rng) -> io::Result<()> {
    let mut output = io::stdout().lock();
    for line in source.lines() {
        let raw = line?;
        eval(&raw, rng, &mut output)?;
    }
    Ok(())
}

fn eval(raw: &str, rng: &mut impl Rng, mut output: impl io::Write) -> io::Result<()> {
    match roll::eval(raw, rng) {
        Ok(value) => writeln!(output, "{value}"),
        Err(error) => writeln!(io::stderr(), "Error: {error}"),
    }
}

struct Seed([u8; 32]);

impl FromArg for Seed {
    type Parent = Box<str>;
    type Error = ();

    fn from_arg(arg: Self::Parent) -> Result<Self, Self::Error> {
        let mut bytes: Box<[u8; 64]> = arg.into_boxed_bytes().try_into().map_err(|_| ())?;
        for byte in bytes.iter_mut() {
            *byte = match byte {
                b'0'..=b'9' => *byte - b'0',
                b'A'..=b'F' => *byte - b'A' + 10,
                b'a'..=b'f' => *byte - b'a' + 10,
                _ => return Err(()),
            }
        }
        let seed: [u8; 32] = std::array::from_fn(|index| {
            let high = bytes[2 * index];
            let low = bytes[2 * index + 1];
            16 * high + low
        });
        Ok(Self(seed))
    }

    fn box_error((): Self::Error) -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(fcla::TypeError("seed"))
    }
}

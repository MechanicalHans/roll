use std::fmt;

use super::RealizeOverFiltered;

impl fmt::Display for super::Realize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Under(inner) => write!(f, "{inner}"),
            Self::OverDropped(inner) => write!(f, "{inner}"),
            Self::OverFiltered(inner) => write!(f, "{inner}"),
        }
    }
}

impl fmt::Display for super::RealizeUnder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            least,
            lesser,
            middle,
            greater,
            greatest,
        } = self;

        if let Some(least) = least {
            write!(f, "[")?;
            display_spaced(&**least, f)?;
        }
        write!(f, "[")?;
        if let Some(lesser) = lesser {
            display_spaced(&**lesser, f)?;
            write!(f, "<")?;
        }
        display_spaced(&**middle, f)?;
        if let Some(greater) = greater {
            write!(f, ">")?;
            display_spaced(&**greater, f)?;
        }
        write!(f, "]")?;
        if let Some(greatest) = greatest {
            display_spaced(&**greatest, f)?;
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl fmt::Display for super::RealizeOverDropped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            least,
            middle,
            greatest,
        } = self;

        write!(f, "[")?;
        if let Some(least) = least {
            display_spaced(&**least, f)?;
            write!(f, "]")?;
        }
        display_spaced(&**middle, f)?;
        if let Some(greatest) = greatest {
            write!(f, "[")?;
            display_spaced(&**greatest, f)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl fmt::Display for RealizeOverFiltered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            least,
            lesser,
            middle,
            greater,
            greatest,
        } = self;

        if let Some(least) = least {
            write!(f, "[")?;
            display_spaced(&**least, f)?;
        }
        write!(f, "[")?;
        display_spaced(&**lesser, f)?;
        write!(f, ">")?;
        display_spaced(&**middle, f)?;
        write!(f, "<")?;
        display_spaced(&**greater, f)?;
        write!(f, "]")?;
        if let Some(greatest) = greatest {
            display_spaced(&**greatest, f)?;
            write!(f, "]")?;
        }
        Ok(())
    }
}

fn display_spaced<I>(sequence: I, f: &mut fmt::Formatter) -> fmt::Result
where
    I: IntoIterator,
    I::Item: fmt::Display,
{
    let mut iterator = sequence.into_iter();
    if let Some(first) = iterator.next() {
        write!(f, "{first}")?;
    }
    for next in iterator {
        write!(f, " {next}")?;
    }
    Ok(())
}

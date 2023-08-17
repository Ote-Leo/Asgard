#![no_std]
use core::fmt;

const NON_ASCII: char = '.';
type AddressWriter = dyn Fn(&mut dyn fmt::Write, usize) -> fmt::Result;

/// Reference wrapper for use in arguments formatting.
pub struct Hex<'a, T: 'a + ?Sized>(&'a T, HexConfig);

impl<'a, T: 'a + AsRef<[u8]>> fmt::Display for Hex<'a, T> {
    /// Formats the value by `simple_hex_write` using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hex_write(f, self.0, self.1.to_simple())
    }
}

impl<'a, T: 'a + AsRef<[u8]>> fmt::Debug for Hex<'a, T> {
    /// Formats the value by `pretty_hex_write` using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hex_write(f, self.0, self.1)
    }
}

/// Configuration parameters for hex dump
#[derive(Clone, Copy, Debug)]
pub struct HexConfig {
    /// Write first line header with data length.
    pub title: bool,
    /// Append ASCII representation column.
    pub ascii: bool,
    /// Source bytes per row. 0 for single row without address prefix.
    pub width: usize,
    /// Chunks count per group. 0 for single group (column).
    pub group: usize,
    /// Source bytes per chunk (word). 0 for single word.
    pub chunk: usize,
    /// Maximum bytes to print.
    pub max_bytes: usize,
    /// Offset added to displayed address prefix.
    pub display_offset: usize,
}

/// Default configuration with `title`, `ascii`, 16 source bytes `width` grouped to 4
/// separate hex bytes. Using `pretty_hex`, `pretty_hex_write` and `fmt::Debug`
/// implementation.
impl Default for HexConfig {
    fn default() -> HexConfig {
        HexConfig {
            title: true,
            ascii: true,
            width: 16,
            group: 4,
            chunk: 1,
            max_bytes: usize::MAX,
            display_offset: 0,
        }
    }
}

impl HexConfig {
    /// Returns configuration for `simple_hex`, `simple_hex_writer` and `fmt::Display`
    /// implementation.
    pub fn simple() -> Self {
        HexConfig::default().to_simple()
    }

    fn delimiter(&self, i: usize) -> &'static str {
        if i > 0 && self.chunk > 0 && i % self.chunk == 0 {
            if self.group > 0 && i % (self.group * self.chunk) == 0 {
                "  "
            } else {
                " "
            }
        } else {
            ""
        }
    }

    fn to_simple(self) -> Self {
        HexConfig {
            title: false,
            ascii: false,
            width: 0,
            ..self
        }
    }
}

/// Allows generates hex dumps to a formatter.
pub trait HexPP {
    /// Wrap self reference for use in `std::fmt::Display` and `std::fmt::Debug`
    /// formatting as hex dumps.
    fn hex_dump(&self) -> Hex<Self>;

    /// Wrap self reference for use in `std::fmt::Display` and `std::fmt::Debug`
    /// formatting as hex dumps in _specified format_.
    fn hex_conf(&self, cfg: HexConfig) -> Hex<Self>;
}

impl<T> HexPP for T
where
    T: AsRef<[u8]> + ?Sized,
{
    fn hex_dump(&self) -> Hex<Self> {
        Hex(&self, HexConfig::default())
    }

    fn hex_conf(&self, cfg: HexConfig) -> Hex<Self> {
        Hex(self, cfg)
    }
}

/// Dump `source` as hex octets in default format wihtout header and ASCII column to the
/// `writer`.
pub fn simple_hex_write<T, W>(writer: &mut W, source: &T) -> fmt::Result
where
    T: AsRef<[u8]> + ?Sized,
    W: fmt::Write,
{
    hex_write(writer, source, HexConfig::simple())
}

/// Write multi-line hexdump in default format complete with addressing, hex digits, and
/// ASCII representation to the `writer`.
pub fn pretty_hex_write<T, W>(writer: &mut W, source: &T) -> fmt::Result
where
    T: AsRef<[u8]>,
    W: fmt::Write,
{
    hex_write(writer, source, HexConfig::default())
}

fn get_address_writer(max_addr: usize) -> &'static AddressWriter {
    match max_addr {
        0x0000..=0xFFFF => &|w, a| write!(w, "{:04x}:    ", a),
        0x010000..=0xFFFFFF => &|w, a| write!(w, "{:06x}:    ", a),
        0x01000000..=0xFFFFFFFF => &|w, a| write!(w, "{:08x}:    ", a),
        _ => &|w, a| write!(w, "{:16x}:    ", a),
    }
}

/// Write hex dump in specified format.
pub fn hex_write<T, W>(writer: &mut W, source: &T, cfg: HexConfig) -> fmt::Result
where
    T: AsRef<[u8]> + ?Sized,
    W: fmt::Write,
{
    let mut source = source.as_ref();
    if cfg.title {
        writeln!(writer, "Length: {0} (0x{0:x})", source.len())?;
    }

    if source.is_empty() {
        return Ok(());
    }

    let omitted = source.len().checked_sub(cfg.max_bytes);
    if omitted.is_some() {
        source = &source[..cfg.max_bytes]
    }

    let lines = source.chunks(if cfg.width > 0 {
        cfg.width
    } else {
        source.len()
    });
    
    let lines_len = lines.len();

    let max_address = if source.len() <= cfg.width {
        source.len() + cfg.display_offset
    } else {
        source.len() - cfg.width + cfg.display_offset
    };

    let write_address = get_address_writer(max_address);

    for (i, row) in lines.enumerate() {
        if cfg.width > 0 {
            write_address(writer, i * cfg.width + cfg.display_offset)?;
        }

        for (i, x) in row.as_ref().iter().enumerate() {
            write!(writer, "{}{:02x}", cfg.delimiter(i), x)?;
        }

        if cfg.ascii {
            for j in row.len() .. cfg.width {
                write!(writer, "{}  ", cfg.delimiter(j))?;
            }

            write!(writer, "    ")?;
            for x in row {
                if x.is_ascii() && !x.is_ascii_control() {
                    writer.write_char((*x).into())?;
                } else {
                    writer.write_char(NON_ASCII)?;
                }
            }
        }

        if i + 1 < lines_len {
            writeln!(writer)?;
        }
    }

    if let Some(o) = omitted {
        write!(writer, "\n...{0} (0x{0:x}) bytes not shown...", o)?;
    }

    Ok(())
}

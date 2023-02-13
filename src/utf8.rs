use binrw::{BinRead, BinWrite, Endian, VecArgs};

pub struct ModifiedUtf8(String);

impl BinRead for ModifiedUtf8 {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let count = u16::read_options(reader, endian, ())? as usize;
        let data = Vec::<u8>::read_options(reader, endian, VecArgs { count, inner: () })?;
        // TODO: Check validity
        Ok(Self(String::from_utf8(data).map_err(|_| {
            binrw::Error::Custom {
                pos: 0,
                err: Box::new("Failed to read utf8 string."),
            }
        })?))
    }
}

impl BinWrite for ModifiedUtf8 {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let bytes = self.0.as_bytes();
        (bytes.len() as u16).write_options(writer, endian, ())?;
        bytes.write_options(writer, endian, ())?;

        Ok(())
    }
}

impl ToString for ModifiedUtf8 {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<String> for ModifiedUtf8 {
    type Error = binrw::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // TODO: check validity
        Ok(Self(value))
    }
}

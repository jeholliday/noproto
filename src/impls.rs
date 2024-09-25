use crate::read::ByteReader;
use crate::write::ByteWriter;
use crate::{
    Fixed32, Fixed64, Int32, Int64, Message, Oneof, OptionalMessage, ReadError, RepeatedMessage, SFixed32, SFixed64,
    WireType, WriteError,
};

impl Message for bool {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_varuint32(*self as _)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        let val = r.read_varuint32()?;

        *self = match val {
            0 => false,
            1 => true,
            _ => return Err(ReadError),
        };
        Ok(())
    }
}

impl Message for u32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_varuint32(*self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        *self = r.read_varuint32()?;
        Ok(())
    }
}

impl Message for u64 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_varuint64(*self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        *self = r.read_varuint64()?;
        Ok(())
    }
}

impl Message for i32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_sint32(*self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        *self = r.read_sint32()?;
        Ok(())
    }
}

impl Message for i64 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_sint64(*self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        *self = r.read_sint64()?;
        Ok(())
    }
}

impl Message for f32 {
    const WIRE_TYPE: WireType = WireType::Fixed32;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_f32(*self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        *self = r.read_f32()?;
        Ok(())
    }
}

impl Message for f64 {
    const WIRE_TYPE: WireType = WireType::Fixed64;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_f64(*self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        *self = r.read_f64()?;
        Ok(())
    }
}

impl Message for Fixed32 {
    const WIRE_TYPE: WireType = WireType::Fixed32;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_u32(self.value)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        self.value = r.read_u32()?;
        Ok(())
    }
}

impl Message for Fixed64 {
    const WIRE_TYPE: WireType = WireType::Fixed64;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_u64(self.value)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        self.value = r.read_u64()?;
        Ok(())
    }
}

impl Message for SFixed32 {
    const WIRE_TYPE: WireType = WireType::Fixed32;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_i32(self.value)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        self.value = r.read_i32()?;
        Ok(())
    }
}

impl Message for SFixed64 {
    const WIRE_TYPE: WireType = WireType::Fixed64;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_i64(self.value)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        self.value = r.read_i64()?;
        Ok(())
    }
}

impl Message for Int32 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_varint32(self.value)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        self.value = r.read_varint32()?;
        Ok(())
    }
}

impl Message for Int64 {
    const WIRE_TYPE: WireType = WireType::Varint;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write_varint64(self.value)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        self.value = r.read_varint64()?;
        Ok(())
    }
}

impl<const N: usize> Message for heapless::String<N> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write(self.as_bytes())
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        let data = r.read_to_end()?;
        let data = core::str::from_utf8(data).map_err(|_| ReadError)?;
        self.clear();
        self.push_str(data).map_err(|_| ReadError)?;
        Ok(())
    }
}

impl<const N: usize> Message for heapless::Vec<u8, N> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        w.write(self)
    }
    fn read_raw(&mut self, r: &mut ByteReader) -> Result<(), ReadError> {
        let data = r.read_to_end()?;
        self.clear();
        self.extend_from_slice(data).map_err(|_| ReadError)?;
        Ok(())
    }
}

impl<M: Message + Default, const N: usize> RepeatedMessage for heapless::Vec<M, N> {
    type Message = M;

    type Iter<'a> = core::slice::Iter<'a, M> where Self: 'a ;

    fn iter(&self) -> Result<Self::Iter<'_>, WriteError> {
        Ok(self[..].iter())
    }

    fn append(&mut self, m: Self::Message) -> Result<(), ReadError> {
        self.push(m).map_err(|_| ReadError)
    }
}

impl<M: Message + Default> OptionalMessage for Option<M> {
    type Message = M;

    fn get(&self) -> Option<&Self::Message> {
        self.as_ref()
    }

    fn set(&mut self, m: Self::Message) -> Result<(), ReadError> {
        *self = Some(m);
        Ok(())
    }
}

impl<M: Oneof> Oneof for Option<M> {
    fn write_raw(&self, w: &mut ByteWriter) -> Result<(), WriteError> {
        if let Some(x) = self {
            x.write_raw(w)?;
        }
        Ok(())
    }

    fn read_raw(&mut self, r: crate::encoding::FieldReader) -> Result<(), ReadError> {
        M::read_raw_option(self, r)
    }

    fn read_raw_option(_this: &mut Option<Self>, _r: crate::encoding::FieldReader) -> Result<(), ReadError> {
        panic!("cannot nest options with oneof.")
    }
}

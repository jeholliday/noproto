use super::*;
use heapless::Vec;

fn packed<M: Message + Default + Eq + core::fmt::Debug>(test_vec: Vec<M, 4>) {
    let mut buf: [u8; 16] = [0; 16];

    let mut w = ByteWriter::new(&mut buf);
    w.write_packed(1, &test_vec).unwrap();

    let mut r = ByteReader::new(&buf);
    let mut fields = r.read_fields();

    let field = fields.next().unwrap().unwrap();
    assert_eq!(field.tag(), 1);
    let mut out_vec = Vec::<M, 10>::new();
    field.read_packed(&mut out_vec).unwrap();
    assert_eq!(out_vec, test_vec);
}

#[test]
fn test_packed_encoding() {
    packed(Vec::<u32, 4>::from_slice(&[1, 2, 3, 4]).unwrap());
    packed(Vec::<u32, 4>::from_slice(&[0xDEADBEEF, 0xAA, 0x0, 0xABCDEF]).unwrap());
    packed(Vec::<i32, 4>::from_slice(&[-1, 0, 1, 2]).unwrap());
}

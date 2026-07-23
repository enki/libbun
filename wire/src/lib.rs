use std::io;
use std::io::Read;
use std::io::Write;

pub const VERSION: u16 = 1;
pub const REQUEST_MAGIC: [u8; 4] = *b"LBPE";
pub const TERMINAL_MAGIC: [u8; 4] = *b"LBPT";
pub const MAX_REQUEST_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_CANDIDATE_BYTES: usize = 16 * 1024 * 1024;
pub const TERMINAL_HEADER_LEN: usize = 11;

pub struct DriveRequest {
    pub prepared_artifact: Vec<u8>,
    pub selected_export: String,
    pub opaque_invocation: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkerFaultKind {
    Preparation = 1,
    InputLowering = 2,
    JavaScriptRejection = 3,
    CargoExtraction = 4,
    Internal = 5,
}

pub fn encode_drive_material(request: DriveRequest) -> Vec<u8> {
    let export = request.selected_export.into_bytes();
    let capacity = request
        .prepared_artifact
        .len()
        .saturating_add(export.len())
        .saturating_add(request.opaque_invocation.len())
        .saturating_add(24);
    let mut bytes = Vec::with_capacity(capacity);
    for field in [request.prepared_artifact, export, request.opaque_invocation] {
        bytes.extend_from_slice(&(field.len() as u64).to_be_bytes());
        bytes.extend_from_slice(&field);
    }
    bytes
}

pub fn write_drive_request(writer: &mut impl Write, material: &[u8]) -> io::Result<()> {
    if material.len() > MAX_REQUEST_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "prepared-export request exceeds internal wire limit",
        ));
    }
    writer.write_all(&REQUEST_MAGIC)?;
    writer.write_all(&VERSION.to_be_bytes())?;
    writer.write_all(&(material.len() as u32).to_be_bytes())?;
    writer.write_all(material)?;
    writer.flush()
}

pub fn read_drive_request(reader: &mut impl Read) -> io::Result<DriveRequest> {
    let mut header = [0_u8; 10];
    reader.read_exact(&mut header)?;
    if header[..4] != REQUEST_MAGIC {
        return Err(invalid("prepared-export request has invalid magic"));
    }
    let version = u16::from_be_bytes([header[4], header[5]]);
    if version != VERSION {
        return Err(invalid(format!(
            "prepared-export request version {version} does not match {VERSION}"
        )));
    }
    let length = u32::from_be_bytes([header[6], header[7], header[8], header[9]]) as usize;
    if length > MAX_REQUEST_BYTES {
        return Err(invalid(
            "prepared-export request exceeds internal wire limit",
        ));
    }
    let mut material = vec![0_u8; length];
    reader.read_exact(&mut material)?;
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra)? != 0 {
        return Err(invalid("worker received more than one drive request"));
    }
    decode_drive_material(&material)
}

pub fn write_cargo(writer: &mut impl Write, cargo: &[u8]) -> io::Result<()> {
    write_candidate(writer, 0, cargo)
}

pub fn write_fault(
    writer: &mut impl Write,
    kind: WorkerFaultKind,
    diagnostic: &str,
) -> io::Result<()> {
    write_candidate(writer, kind as u8, diagnostic.as_bytes())
}

fn write_candidate(writer: &mut impl Write, kind: u8, payload: &[u8]) -> io::Result<()> {
    if payload.len() > MAX_CANDIDATE_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "prepared-export terminal candidate exceeds internal wire limit",
        ));
    }
    writer.write_all(&TERMINAL_MAGIC)?;
    writer.write_all(&VERSION.to_be_bytes())?;
    writer.write_all(&[kind])?;
    writer.write_all(&(payload.len() as u32).to_be_bytes())?;
    writer.write_all(payload)?;
    writer.flush()
}

fn decode_drive_material(material: &[u8]) -> io::Result<DriveRequest> {
    let mut cursor = 0;
    let prepared_artifact = take_field(material, &mut cursor)?;
    let selected_export = String::from_utf8(take_field(material, &mut cursor)?).map_err(invalid)?;
    let opaque_invocation = take_field(material, &mut cursor)?;
    if cursor != material.len() {
        return Err(invalid("prepared-export request has trailing material"));
    }
    Ok(DriveRequest {
        prepared_artifact,
        selected_export,
        opaque_invocation,
    })
}

fn take_field(material: &[u8], cursor: &mut usize) -> io::Result<Vec<u8>> {
    let length_end = cursor
        .checked_add(8)
        .filter(|end| *end <= material.len())
        .ok_or_else(|| invalid("prepared-export request field length is truncated"))?;
    let length = u64::from_be_bytes(
        material[*cursor..length_end]
            .try_into()
            .expect("field length slice is exact"),
    );
    let length = usize::try_from(length)
        .map_err(|_| invalid("prepared-export request field length exceeds address space"))?;
    *cursor = length_end;
    let field_end = cursor
        .checked_add(length)
        .filter(|end| *end <= material.len())
        .ok_or_else(|| invalid("prepared-export request field is truncated"))?;
    let field = material[*cursor..field_end].to_vec();
    *cursor = field_end;
    Ok(field)
}

fn invalid(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trips_opaque_fields() {
        let material = encode_drive_material(DriveRequest {
            prepared_artifact: vec![0, 255, 1],
            selected_export: "renamed-export".to_string(),
            opaque_invocation: b"not tson\0".to_vec(),
        });
        let mut framed = Vec::new();
        write_drive_request(&mut framed, &material).expect("request writes");
        let decoded = read_drive_request(&mut framed.as_slice()).expect("request reads");
        assert_eq!(decoded.prepared_artifact, vec![0, 255, 1]);
        assert_eq!(decoded.selected_export, "renamed-export");
        assert_eq!(decoded.opaque_invocation, b"not tson\0");
    }
}

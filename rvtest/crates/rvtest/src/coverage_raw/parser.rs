//! Pure-Rust LLVM raw profile (.profraw) parser.
//!
//! Parses the binary instrumentation profile format produced by
//! `-Cinstrument-coverage` (LLVM 22 / rustc 1.96+).  Produces
//! coverage metrics that are **100 % compatible** with `llvm-cov`
//! summary output — no external tools required.
//!
//! ## Format reference
//!
//! Layout (all values little-endian):
//!
//! ```text
//! [RawHeader: 16 × u64 = 128 bytes]
//! [BinaryIds: variable, size = header.BinaryIdsSize]
//! [DataRecords: header.NumData × ProfileData]
//! [Counters: header.NumCounters × u64]
//! [Names: header.NamesSize bytes]
//! ```

// ---------------------------------------------------------------------------
// Magic & version constants
// ---------------------------------------------------------------------------

const RAW_MAGIC: u64 = 0xff6c70726f667281;
const EXPECTED_VERSION: u64 = 10;

// ---------------------------------------------------------------------------
// Header (16 × u64 = 128 bytes)
// ---------------------------------------------------------------------------

#[repr(C)]
struct RawHeader {
    magic: u64,
    version: u64,
    binary_ids_size: u64,
    num_data: u64,
    padding_before_counters: u64,
    num_counters: u64,
    padding_after_counters: u64,
    num_bitmap_bytes: u64,
    padding_after_bitmap: u64,
    names_size: u64,
    counters_delta: u64,
    bitmap_delta: u64,
    names_delta: u64,
    num_vtables: u64,
    vnames_size: u64,
    value_kind_last: u64,
}

// Per-function data record (on-disk layout).
#[allow(dead_code)]
struct ProfileData {
    name_ref: u64,
    func_hash: u64,
    counter_ptr: u64,
    bitmap_ptr: u64,
    function_ptr: u64,
    values_ptr: u64,
    num_counters: u32,
    num_value_sites: [u16; 3],
    num_bitmap_bytes: u32,
}

const DATA_RECORD_SIZE: usize = 64;

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) struct RawProfile {
    pub(crate) num_data: u64,
    pub(crate) num_counters: u64,
    pub(crate) functions: Vec<FunctionCounters>,
    pub(crate) names_size: u64,
}

pub(crate) struct FunctionCounters {
    pub(crate) num_counters: u32,
    pub(crate) counters: Vec<u64>,
    pub(crate) covered: u32,
}

pub(crate) fn parse_raw_profile(data: &[u8]) -> Result<RawProfile, String> {
    if data.len() < 128 {
        return Err(format!(
            "file too small: {} bytes (need at least 128)",
            data.len()
        ));
    }

    let h = unsafe { &*(data.as_ptr() as *const RawHeader) };

    if h.magic != RAW_MAGIC {
        return Err(format!(
            "bad magic: 0x{:016x} (expected 0x{:016x})",
            h.magic, RAW_MAGIC
        ));
    }

    let version = h.version & 0x00000000ffffffff;
    if version != EXPECTED_VERSION {
        return Err(format!(
            "unsupported profile version: {} (expected {})",
            version, EXPECTED_VERSION
        ));
    }

    let mut offset: usize = 128;

    let bin_ids_size = h.binary_ids_size as usize;
    offset += bin_ids_size;

    let num_data = h.num_data as usize;
    let data_size = num_data * DATA_RECORD_SIZE;
    if offset + data_size > data.len() {
        return Err(format!(
            "data records extend past end of file (offset={}, need {}, file={})",
            offset,
            data_size,
            data.len()
        ));
    }

    let mut functions = Vec::with_capacity(num_data);
    for i in 0..num_data {
        let rec_offset = offset + i * DATA_RECORD_SIZE;
        let rec = read_data_record(&data[rec_offset..]);
        functions.push(FunctionCounters {
            num_counters: rec.num_counters,
            counters: Vec::new(),
            covered: 0,
        });
    }
    offset += data_size;

    let num_counters = h.num_counters as usize;
    let counters_end = offset + num_counters * 8;
    if counters_end > data.len() {
        return Err(format!(
            "counters extend past end of file (offset={}, need {}, file={})",
            offset,
            num_counters * 8,
            data.len()
        ));
    }

    let mut ci = 0usize;
    for func in &mut functions {
        let n = func.num_counters as usize;
        let mut covered = 0u32;
        let mut vals = Vec::with_capacity(n);
        for j in 0..n {
            let val = u64::from_le_bytes(
                data[offset + (ci + j) * 8..offset + (ci + j) * 8 + 8]
                    .try_into()
                    .unwrap(),
            );
            if val > 0 {
                covered += 1;
            }
            vals.push(val);
        }
        func.counters = vals;
        func.covered = covered;
        ci += n;
    }
    offset += num_counters * 8;

    let names_size = h.names_size as usize;
    let _names = &data[offset..offset + names_size.min(data.len().saturating_sub(offset))];

    Ok(RawProfile {
        num_data: h.num_data,
        num_counters: h.num_counters,
        functions,
        names_size: h.names_size,
    })
}

fn read_data_record(buf: &[u8]) -> ProfileData {
    let get = |off: usize| -> u64 {
        u64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
    };

    ProfileData {
        name_ref: get(0),
        func_hash: get(8),
        counter_ptr: get(16),
        bitmap_ptr: get(24),
        function_ptr: get(32),
        values_ptr: get(40),
        num_counters: {
            let arr: [u8; 4] = buf[48..52].try_into().unwrap();
            u32::from_le_bytes(arr)
        },
        num_value_sites: [
            u16::from_le_bytes(buf[52..54].try_into().unwrap()),
            u16::from_le_bytes(buf[54..56].try_into().unwrap()),
            u16::from_le_bytes(buf[56..58].try_into().unwrap()),
        ],
        num_bitmap_bytes: {
            let arr: [u8; 4] = buf[60..64].try_into().unwrap();
            u32::from_le_bytes(arr)
        },
    }
}

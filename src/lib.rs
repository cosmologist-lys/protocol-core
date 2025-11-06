pub mod core;
pub mod defi;
pub mod digester;
pub mod utils;

pub use crate::core::{
    DirectionEnum, MsgTypeEnum, Symbol,
    cache::ProtocolCache,
    parts::{
        placeholder::PlaceHolder,
        raw_capsule::RawCapsule,
        raw_chamber::RawChamber,
        rawfield::Rawfield,
        traits::{
            AutoDecoding, AutoDecodingParams, AutoEncoding, AutoEncodingParams, Cmd,
            ProtocolConfig, Transport,
        },
        transport_carrier::TransportCarrier,
        transport_pair::TransportPair,
    },
    reader::Reader,
    type_converter::{
        FieldCompareDecoder, FieldConvertDecoder, FieldEnumDecoder, FieldTranslator, FieldType,
        TryFromBytes,
    },
    writer::Writer,
};
pub use crate::defi::{
    ProtocolResult,
    bridge::{
        /* JarDecodeResponse, JarEncodeRequest, JarEncodeResponse, */ JniRequest, JniResponse,
        ReportField,
    },
    crc_enum::CrcType,
    error::{
        ProtocolError, comm_error::CommError, hex_digest_error::HexDigestError, hex_error::HexError,
    },
};
pub use crate::utils::{crc_util, generate_rand, hex_util, math_util, timestamp_util, to_pinyin};

pub use crate::digester::{aes_digester, md5_digester};

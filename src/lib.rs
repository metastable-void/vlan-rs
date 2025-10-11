use std::{
    fmt::{Debug, Display},
    hash::Hash,
    num::NonZero,
};

pub type RawVlanId = u16;

/// Types that can be converted to a raw VLAN ID (u16).
pub trait IntoRawVlanId {
    /// Get the u16 value.
    fn into_raw_vlan_id(&self) -> RawVlanId;

    /// Convert the type to a big-endian bytes.
    ///
    /// You don't need to implement this method yourself.
    fn into_be_bytes(&self) -> [u8; 2] {
        u16::to_be_bytes(self.into_raw_vlan_id())
    }
}

/// The error value that represents an invalid VLAN ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidVlanId;

impl Display for InvalidVlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("INVALID_VLAN_ID")
    }
}

impl std::error::Error for InvalidVlanId {}

/// The value that represents the native VLAN.
///
/// It is semantivally zero.
/// Internally, it consumes no memory.
#[derive(Debug, Clone, Copy)]
pub struct NativeVlanId;

impl NativeVlanId {
    pub const VALUE: u16 = 0;
}

impl Display for NativeVlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0")
    }
}

impl IntoRawVlanId for NativeVlanId {
    fn into_raw_vlan_id(&self) -> RawVlanId {
        Self::VALUE
    }
}

impl Into<u16> for NativeVlanId {
    fn into(self) -> u16 {
        Self::VALUE
    }
}

impl Into<u16> for &NativeVlanId {
    fn into(self) -> u16 {
        (*self).into()
    }
}

impl TryFrom<u16> for NativeVlanId {
    type Error = InvalidVlanId;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == Self::VALUE {
            Ok(NativeVlanId)
        } else {
            Err(InvalidVlanId)
        }
    }
}

impl<Rhs: IntoRawVlanId> PartialEq<Rhs> for NativeVlanId {
    fn eq(&self, other: &Rhs) -> bool {
        self.into_raw_vlan_id() == other.into_raw_vlan_id()
    }
}

impl<Rhs: IntoRawVlanId> PartialOrd<Rhs> for NativeVlanId {
    fn partial_cmp(&self, other: &Rhs) -> Option<std::cmp::Ordering> {
        Some(self.into_raw_vlan_id().cmp(&other.into_raw_vlan_id()))
    }
}

impl Eq for NativeVlanId {}

impl Hash for NativeVlanId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.into());
    }
}

/// Non-zero VLAN ID valid per the IEEE 802.1Q standard.
///
/// Range: 1-4094
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VlanId {
    inner: NonZero<u16>,
}

impl VlanId {
    pub const MIN_VALUE: u16 = 1;
    pub const MAX_VALUE: u16 = 4094;

    /// Minimal tagged VLAN ID (1)
    pub const MIN: Self = unsafe { Self::new_unchecked(Self::MIN_VALUE) };

    /// Maximum tagged VLAN ID (4094)
    pub const MAX: Self = unsafe { Self::new_unchecked(Self::MAX_VALUE) };

    /// Same as MIN (1)
    pub const ONE: Self = Self::MIN;

    pub const fn try_new(vlan: u16) -> Result<Self, InvalidVlanId> {
        if vlan < Self::MIN_VALUE || vlan > Self::MAX_VALUE {
            Err(InvalidVlanId)
        } else {
            Ok(Self {
                inner: unsafe { NonZero::new_unchecked(vlan) },
            })
        }
    }

    pub(crate) const unsafe fn new_unchecked(value: u16) -> Self {
        Self {
            inner: unsafe { NonZero::new_unchecked(value) },
        }
    }

    pub const fn inner(&self) -> NonZero<u16> {
        self.inner
    }

    pub const fn as_u16(&self) -> u16 {
        self.inner.get()
    }
}

impl Display for VlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.as_u16()))
    }
}

impl TryFrom<u16> for VlanId {
    type Error = InvalidVlanId;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl Into<u16> for VlanId {
    fn into(self) -> u16 {
        self.inner.get()
    }
}

impl Into<u16> for &VlanId {
    fn into(self) -> u16 {
        (*self).into()
    }
}

impl Debug for VlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("VlanId({})", self.inner.get()))
    }
}

impl IntoRawVlanId for VlanId {
    fn into_raw_vlan_id(&self) -> RawVlanId {
        self.inner.get()
    }
}

impl<Rhs: IntoRawVlanId> PartialEq<Rhs> for VlanId {
    fn eq(&self, other: &Rhs) -> bool {
        self.into_raw_vlan_id() == other.into_raw_vlan_id()
    }
}

impl<Rhs: IntoRawVlanId> PartialOrd<Rhs> for VlanId {
    fn partial_cmp(&self, other: &Rhs) -> Option<std::cmp::Ordering> {
        Some(self.into_raw_vlan_id().cmp(&other.into_raw_vlan_id()))
    }
}

impl Ord for VlanId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_raw_vlan_id().cmp(&other.into_raw_vlan_id())
    }
}

impl Eq for VlanId {}

impl Hash for VlanId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.into());
    }
}

/// VLAN ID that may be a native VLAN.
///
/// Effectively, this is an `Option<T>` for the VLAN world.
///
/// It has the same memory layout as `u16`.
#[derive(Clone, Copy, Debug)]
pub enum MaybeVlanId {
    /// Native VLAN value. It is effectively the same as `Option::None`.
    Native(NativeVlanId),

    /// Tagged valid VLAN value.
    Tagged(VlanId),
}

impl MaybeVlanId {
    /// The size of this type in octets
    pub const OCTET_SIZE: usize = 2;

    /// Number of bits this type contains
    pub const BITS: u32 = 16;

    /// Number of bits actually used
    pub const EFFECTIVE_BITS: u32 = 12;

    /// Native VLAN value
    pub const NATIVE: MaybeVlanId = MaybeVlanId::Native(NativeVlanId);

    pub const MIN_TAGGED_VLAN: MaybeVlanId = MaybeVlanId::Tagged(VlanId::MIN);
    pub const MAX_TAGGED_VLAN: MaybeVlanId = MaybeVlanId::Tagged(VlanId::MAX);

    /// Tagged VLAN value
    pub const fn tagged(vlan: VlanId) -> Self {
        Self::Tagged(vlan)
    }

    pub const fn as_u16(&self) -> u16 {
        match *self {
            Self::Tagged(vlan) => vlan.as_u16(),
            _ => 0,
        }
    }

    pub const fn try_new(vlan: u16) -> Result<Self, InvalidVlanId> {
        match vlan {
            0 => Ok(Self::NATIVE),
            VlanId::MIN_VALUE..=VlanId::MAX_VALUE => unsafe {
                Ok(Self::Tagged(VlanId::new_unchecked(vlan)))
            },
            _ => Err(InvalidVlanId),
        }
    }
}

impl Display for MaybeVlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.as_u16()))
    }
}

impl TryFrom<u16> for MaybeVlanId {
    type Error = InvalidVlanId;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl Into<u16> for MaybeVlanId {
    fn into(self) -> u16 {
        self.as_u16()
    }
}

impl Into<u16> for &MaybeVlanId {
    fn into(self) -> u16 {
        self.as_u16()
    }
}

impl IntoRawVlanId for MaybeVlanId {
    fn into_raw_vlan_id(&self) -> RawVlanId {
        match *self {
            Self::Native(_) => 0,
            Self::Tagged(vlan) => vlan.into_raw_vlan_id(),
        }
    }
}

impl<Rhs: IntoRawVlanId> PartialEq<Rhs> for MaybeVlanId {
    fn eq(&self, other: &Rhs) -> bool {
        self.into_raw_vlan_id() == other.into_raw_vlan_id()
    }
}

impl<Rhs: IntoRawVlanId> PartialOrd<Rhs> for MaybeVlanId {
    fn partial_cmp(&self, other: &Rhs) -> Option<std::cmp::Ordering> {
        Some(self.into_raw_vlan_id().cmp(&other.into_raw_vlan_id()))
    }
}

impl Eq for MaybeVlanId {}

impl Ord for MaybeVlanId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_raw_vlan_id().cmp(&other.into_raw_vlan_id())
    }
}

impl Hash for MaybeVlanId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.into());
    }
}

#[cfg(feature = "serde")]
mod serialization {
    use super::*;
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::fmt;

    struct VlanIdVisitor;

    impl<'de> Visitor<'de> for VlanIdVisitor {
        type Value = VlanId;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Expected a valid tagged VLAN ID")
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.try_into().map_err(|e| de::Error::custom(e))
        }
    }

    impl<'de> Deserialize<'de> for VlanId {
        fn deserialize<D>(deserializer: D) -> Result<VlanId, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(VlanIdVisitor)
        }
    }

    impl Serialize for VlanId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u16(self.as_u16())
        }
    }

    struct MaybeVlanIdVisitor;

    impl<'de> Visitor<'de> for MaybeVlanIdVisitor {
        type Value = MaybeVlanId;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Expected a valid tagged VLAN ID or zero (native VLAN)")
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.try_into().map_err(|e| de::Error::custom(e))
        }
    }

    impl<'de> Deserialize<'de> for MaybeVlanId {
        fn deserialize<D>(deserializer: D) -> Result<MaybeVlanId, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(MaybeVlanIdVisitor)
        }
    }

    impl Serialize for MaybeVlanId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u16(self.as_u16())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_check() {
        assert!(TryInto::<VlanId>::try_into(0).is_err());
        assert!(TryInto::<VlanId>::try_into(1).is_ok());
        assert!(TryInto::<VlanId>::try_into(4094).is_ok());
        assert!(TryInto::<VlanId>::try_into(4095).is_err());
        assert!(TryInto::<VlanId>::try_into(u16::MAX).is_err());

        assert_eq!(TryInto::<MaybeVlanId>::try_into(0), Ok(MaybeVlanId::NATIVE));
        assert_eq!(
            TryInto::<MaybeVlanId>::try_into(1),
            Ok(MaybeVlanId::MIN_TAGGED_VLAN)
        );
        assert_eq!(
            TryInto::<MaybeVlanId>::try_into(4094),
            Ok(MaybeVlanId::MAX_TAGGED_VLAN)
        );
        assert!(TryInto::<MaybeVlanId>::try_into(4095).is_err());
        assert!(TryInto::<MaybeVlanId>::try_into(u16::MAX).is_err());

        assert_eq!(MaybeVlanId::NATIVE, NativeVlanId);
    }
}

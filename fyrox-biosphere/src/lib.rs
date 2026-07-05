pub mod alignment;
pub mod bdna;
pub mod capacity;
pub mod container_format;
pub mod domain;
pub mod heraldry;
pub mod wire;

// ── Simulation constants ────────────────────────────────────────────────────

/// Resonance gate threshold. Signals below this value are muted (Stasis state).
pub const RESONANCE_THRESHOLD: f32 = 0.40;

/// Universal slot capacity per container level (2^4).
pub const SLOT_CAPACITY: usize = 16;

// ── Newtypes ────────────────────────────────────────────────────────────────

/// Packed 64-bit B-DNA strand. Each bit encodes one hereditary trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BDNA(pub u64);

impl BDNA {
    pub fn bit(&self, index: usize) -> bool {
        index < 64 && (self.0 >> index) & 1 == 1
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        if index < 64 {
            if value { self.0 |= 1u64 << index; } else { self.0 &= !(1u64 << index); }
        }
    }

    pub fn xor(&self, other: BDNA) -> BDNA {
        BDNA(self.0 ^ other.0)
    }

    pub fn popcount(&self) -> u32 {
        self.0.count_ones()
    }
}

/// Unique spatial voxel identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VoxelID(pub u64);

// ── Trinary logic ───────────────────────────────────────────────────────────

/// Three-valued state for axiom logic gates.
/// Inverse = structural opposition, Stasis = neutral/muted, Thesis = active/aligned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrinaryState {
    Inverse = -1,
    Stasis = 0,
    Thesis = 1,
}

impl TrinaryState {
    pub fn value(self) -> f32 {
        match self {
            TrinaryState::Inverse => -1.0,
            TrinaryState::Stasis => 0.0,
            TrinaryState::Thesis => 1.0,
        }
    }

    pub fn from_i8(v: i8) -> Self {
        match v {
            -1 => TrinaryState::Inverse,
            1 => TrinaryState::Thesis,
            _ => TrinaryState::Stasis,
        }
    }
}

impl Default for TrinaryState {
    fn default() -> Self {
        TrinaryState::Stasis
    }
}

// ── Wire packets (runtime messages) ────────────────────────────────────────

/// Runtime message payload travelling over a wire connection.
/// Coexists with `WireType` (connection semantics) — these are the actual data packets.
#[derive(Debug, Clone)]
pub enum WirePacket {
    /// Raw data payload (universal fallback).
    DAT(u64),
    /// Control signal (boolean/gate/trigger, encoded as f32: 0.0 = off, 1.0 = on).
    CTL(f32),
    /// Resonance signal carrying Harmonic Tensor Graph field values.
    RES { hz: f32, amp: f32 },
    /// Narrative event reference.
    NAR { event_id: u32, key: String },
}

impl WirePacket {
    pub fn wire_type_abbrev(&self) -> &'static str {
        match self {
            WirePacket::DAT(_) => "DAT",
            WirePacket::CTL(_) => "CTL",
            WirePacket::RES { .. } => "RES",
            WirePacket::NAR { .. } => "NAR",
        }
    }
}

// ── Persistence traits ──────────────────────────────────────────────────────

pub trait PersistentState {
    fn lineage_hash(&self) -> u64;
    fn resonance_hz(&self) -> f32;
}

pub trait LineageCrawler {
    fn crawl_ancestry(&self, bdna: BDNA) -> Vec<BDNA>;
}

pub trait ResonanceSync {
    fn sync(&mut self, hz: f32, amp: f32);
}

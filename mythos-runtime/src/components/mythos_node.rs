use fyrox::{
    core::{
        algebra::Vector3,
        reflect::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    graph::SceneGraph,
    plugin::error::GameResult,
    scene::rigidbody::RigidBody,
    script::{ScriptContext, ScriptTrait},
};
use fyrox_biosphere::{RESONANCE_THRESHOLD, TrinaryState};

/// Unified simulation node for the BioSpark narrative engine.
/// One struct merges: resonance, axiom logic, heraldry, B-DNA, and physics response.
/// Attach to any Fyrox scene node to make it participate in the simulation.
#[derive(Visit, PartialEq, Reflect, Debug, Clone)]
#[reflect(type_uuid = "b3d7f8a2-4c6e-4d9f-8b1a-2e5c7f0d3e6b")]
pub struct MythosNodeComponent {
    /// Resonance frequency of this node (Hz).
    #[visit(optional)]
    pub frequency_hz: InheritableVariable<f32>,

    /// Harmonic ratio to parent container.
    #[visit(optional)]
    pub harmonic_ratio: InheritableVariable<f32>,

    /// Resonance threshold. Below RESONANCE_THRESHOLD (0.40) → Stasis; above → Thesis.
    #[visit(optional)]
    pub threshold: InheritableVariable<f32>,

    /// Faction index (0=Luminarite, 1=Venturan, 2=Sylvanid, 3=Hydralis, 4=Syntaran).
    #[visit(optional)]
    pub faction: InheritableVariable<u32>,

    /// B-DNA packed as u64. Each bit encodes a hereditary trait.
    #[visit(optional)]
    pub bdna_bits: InheritableVariable<u64>,

    /// Force scale applied to RigidBody impulses. Modulated by trinary state.
    #[visit(optional)]
    pub force_scale: InheritableVariable<f32>,

    /// Lineage hash for provenance. Computed from bdna_bits on start.
    #[visit(optional)]
    pub lineage_hash: InheritableVariable<u64>,

    /// Current trinary state as i8: -1=Inverse, 0=Stasis, 1=Thesis.
    /// Read-only from inspector — driven by threshold vs RESONANCE_THRESHOLD.
    #[visit(optional)]
    pub trinary_state: InheritableVariable<i8>,
}

impl Default for MythosNodeComponent {
    fn default() -> Self {
        Self {
            frequency_hz: 440.0_f32.into(),
            harmonic_ratio: 1.0_f32.into(),
            threshold: 0.5_f32.into(),
            faction: 2u32.into(), // Sylvanid (neutral) default
            bdna_bits: 0u64.into(),
            force_scale: 1.0_f32.into(),
            lineage_hash: 0u64.into(),
            trinary_state: 0i8.into(),
        }
    }
}

impl ScriptTrait for MythosNodeComponent {
    fn on_start(&mut self, _context: &mut ScriptContext) -> GameResult {
        // Compute lineage hash from bdna_bits using simple FNV-style fold.
        let bdna = *self.bdna_bits;
        *self.lineage_hash = fnv64(bdna);
        // Compute initial trinary state.
        *self.trinary_state = compute_trinary(*self.threshold).to_i8();
        Ok(())
    }

    fn on_update(&mut self, context: &mut ScriptContext) -> GameResult {
        let new_state = compute_trinary(*self.threshold);
        let new_i8 = new_state.to_i8();

        if *self.trinary_state != new_i8 {
            *self.trinary_state = new_i8;
        }

        // If Thesis (above threshold), apply a resonance impulse to any RigidBody on this node.
        if new_state == TrinaryState::Thesis {
            if let Some(body) = context
                .scene
                .graph
                .try_get_node_mut(context.handle)
                .ok()
                .and_then(|n| n.cast_mut::<RigidBody>())
            {
                let scale = *self.force_scale * new_state.value();
                let freq_factor = (*self.frequency_hz / 440.0).clamp(0.1, 10.0);
                body.apply_force(Vector3::new(0.0, scale * freq_factor, 0.0));
            }
        }

        Ok(())
    }
}

/// Receive an external CTL packet — used by EidolonRack sequencer to drive threshold.
impl MythosNodeComponent {
    pub fn receive_ctl(&mut self, value: f32) {
        *self.threshold = value.clamp(0.0, 1.0);
    }

    pub fn receive_res(&mut self, hz: f32, amp: f32) {
        *self.frequency_hz = hz;
        *self.threshold = (*self.threshold + amp).min(1.0);
    }
}

fn compute_trinary(threshold: f32) -> TrinaryState {
    if threshold < RESONANCE_THRESHOLD {
        TrinaryState::Stasis
    } else if threshold >= RESONANCE_THRESHOLD {
        TrinaryState::Thesis
    } else {
        TrinaryState::Inverse
    }
}

fn fnv64(v: u64) -> u64 {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;
    let mut hash = FNV_OFFSET;
    for byte in v.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

trait TrinaryI8 {
    fn to_i8(self) -> i8;
}

impl TrinaryI8 for TrinaryState {
    fn to_i8(self) -> i8 {
        match self {
            TrinaryState::Inverse => -1,
            TrinaryState::Stasis => 0,
            TrinaryState::Thesis => 1,
        }
    }
}

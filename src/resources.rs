use hashbrown::{HashMap, HashSet};
use once_cell::sync::Lazy;
use lost_metrics_core::models::*;

pub static NPC_DATA: Lazy<HashMap<u32, Npc>> = Lazy::new(|| {

    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/Npc.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/Npc.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static SKILL_DATA: Lazy<HashMap<u32, SkillData>> = Lazy::new(|| {

    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/Skill.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/Skill.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static SKILL_EFFECT_DATA: Lazy<HashMap<u32, SkillEffectData>> = Lazy::new(|| {
    
    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/SkillEffect.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/SkillEffect.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static SKILL_BUFF_DATA: Lazy<HashMap<u32, SkillBuffData>> = Lazy::new(|| {

    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/SkillBuff.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/SkillBuff.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static COMBAT_EFFECT_DATA: Lazy<HashMap<u32, CombatEffectData>> = Lazy::new(|| {
  
    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/CombatEffect.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/CombatEffect.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static ENGRAVING_DATA: Lazy<HashMap<u32, EngravingData>> = Lazy::new(|| {
    
    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/Ability.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/Ability.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static ESTHER_DATA: Lazy<Vec<Esther>> = Lazy::new(|| {

    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/Esther.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/Esther.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static VALID_ZONES: Lazy<HashSet<u32>> = Lazy::new(|| {

    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/ValidZones.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/ValidZones.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

pub static STAT_TYPE_MAP: Lazy<HashMap<&'static str, u32>> = Lazy::new(|| {
    
    #[cfg(feature = "json")]
    {
        let bytes = include_bytes!("./data/json/StatTypes.json");
        serde_json::from_slice(bytes).unwrap()
    }

    #[cfg(feature = "msgpack")]
    {
        let bytes = include_bytes!("./data/messagepack/StatTypes.mpk");
        rmp_serde::from_slice(bytes).unwrap()
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_resources {
        ($($data:expr => $test_fn:ident),*) => {
            $(
                #[test]
                fn $test_fn() {
                    let data = &*$data;
                    assert!(!data.is_empty());
                }
            )*
        };
    }

    test_resources!(
        NPC_DATA => should_load_npc_data_without_errors,
        SKILL_DATA => should_load_skill_data_without_errors,
        SKILL_EFFECT_DATA => should_load_skill_effect_data_without_errors,
        SKILL_BUFF_DATA => should_load_skill_buff_data_without_errors,
        COMBAT_EFFECT_DATA => should_load_combat_effect_data_without_errors,
        ENGRAVING_DATA => should_load_engraving_data_without_errors,
        ESTHER_DATA => should_load_esther_data_without_errors,
        VALID_ZONES => should_load_valid_zones_without_errors,
        STAT_TYPE_MAP => should_load_stat_type_map_without_errors
    );
}
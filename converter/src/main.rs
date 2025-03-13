use std::fs::{self, File};
use serde_json::json;
use rmp_serde::{encode, Deserializer, Serializer};
use lost_metrics_data::*;

fn main() {
    let mut buf: Vec<u8> = Vec::new();

    let data = &*NPC_DATA;
    let mut file = File::create("../src/data/messagepack/Npc.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*SKILL_DATA;
    let mut file = File::create("../src/data/messagepack/Skill.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*SKILL_EFFECT_DATA;
    let mut file = File::create("../src/data/messagepack/SkillEffect.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*SKILL_BUFF_DATA;
    let mut file = File::create("../src/data/messagepack/SkillBuff.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*COMBAT_EFFECT_DATA;
    let mut file = File::create("../src/data/messagepack/CombatEffect.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*ENGRAVING_DATA;
    let mut file = File::create("../src/data/messagepack/Ability.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*ESTHER_DATA;
    let mut file = File::create("../src/data/messagepack/Esther.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*VALID_ZONES;
    let mut file = File::create("../src/data/messagepack/ValidZones.mpk").unwrap();
    encode::write(&mut file, data).unwrap();

    let data = &*STAT_TYPE_MAP;
    let mut file = File::create("../src/data/messagepack/StatTypes.mpk").unwrap();
    encode::write(&mut file, data).unwrap();
}
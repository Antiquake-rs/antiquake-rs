// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.



/*


{
"origin" "472 1408 -144"
"classname" "item_spikes"
}


*/
use std::{cell::RefCell, convert::TryInto, error::Error, fmt, rc::Rc, collections::HashMap};

use crate::{
    common::{engine::duration_to_f32, net::EntityState},
    server::{
        
        world::phys::MoveKind,
    },
};

use arrayvec::ArrayString;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cgmath::Vector3;
use chrono::Duration;
use num::FromPrimitive;
use uluru::LRUCache;

use super::WorldError;

pub const MAX_ENT_LEAVES: usize = 16;

pub const STATIC_ADDRESS_COUNT: usize = 105;

#[derive(Debug)]
pub enum EntityError {
    Io(::std::io::Error),
    Address(isize),
    Other(String),
}

impl EntityError {
    pub fn with_msg<S>(msg: S) -> Self
    where
        S: AsRef<str>,
    {
        EntityError::Other(msg.as_ref().to_owned())
    }
}

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EntityError::Io(ref err) => {
                write!(f, "I/O error: ")?;
                err.fmt(f)
            }
            EntityError::Address(val) => write!(f, "Invalid address ({})", val),
            EntityError::Other(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for EntityError {}

impl From<::std::io::Error> for EntityError {
    fn from(error: ::std::io::Error) -> Self {
        EntityError::Io(error)
    }
}

/// A trait which covers addresses of typed values.
pub trait FieldAddr {
    /// The type of value referenced by this address.
    type Value;

    /// Loads the value at this address.
    fn load(&self, ent: &Entity) -> Result<Self::Value, EntityError>;

    /// Stores a value at this address.
    fn store(&self, ent: &mut Entity, value: Self::Value) -> Result<(), EntityError>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum FieldAddrFloat {
    ModelIndex = 0,
    AbsMinX = 1,
    AbsMinY = 2,
    AbsMinZ = 3,
    AbsMaxX = 4,
    AbsMaxY = 5,
    AbsMaxZ = 6,
    /// Used by mobile level geometry such as moving platforms.
    LocalTime = 7,
    /// Determines the movement behavior of an entity. The value must be a variant of `MoveKind`.
    MoveKind = 8,
    Solid = 9,
    OriginX = 10,
    OriginY = 11,
    OriginZ = 12,
    OldOriginX = 13,
    OldOriginY = 14,
    OldOriginZ = 15,
    VelocityX = 16,
    VelocityY = 17,
    VelocityZ = 18,
    AnglesX = 19,
    AnglesY = 20,
    AnglesZ = 21,
    AngularVelocityX = 22,
    AngularVelocityY = 23,
    AngularVelocityZ = 24,
    PunchAngleX = 25,
    PunchAngleY = 26,
    PunchAngleZ = 27,
    /// The index of the entity's animation frame.
    FrameId = 30,
    /// The index of the entity's skin.
    SkinId = 31,
    /// Effects flags applied to the entity. See `EntityEffects`.
    Effects = 32,
    /// Minimum extent in local coordinates, X-coordinate.
    MinsX = 33,
    /// Minimum extent in local coordinates, Y-coordinate.
    MinsY = 34,
    /// Minimum extent in local coordinates, Z-coordinate.
    MinsZ = 35,
    /// Maximum extent in local coordinates, X-coordinate.
    MaxsX = 36,
    /// Maximum extent in local coordinates, Y-coordinate.
    MaxsY = 37,
    /// Maximum extent in local coordinates, Z-coordinate.
    MaxsZ = 38,
    SizeX = 39,
    SizeY = 40,
    SizeZ = 41,
    /// The next server time at which the entity should run its think function.
    NextThink = 46,
    /// The entity's remaining health.
    Health = 48,
    /// The number of kills scored by the entity.
    Frags = 49,
    Weapon = 50,
    WeaponFrame = 52,
    /// The entity's remaining ammunition for its selected weapon.
    CurrentAmmo = 53,
    /// The entity's remaining shotgun shells.
    AmmoShells = 54,
    /// The entity's remaining shotgun shells.
    AmmoNails = 55,
    /// The entity's remaining rockets/grenades.
    AmmoRockets = 56,
    AmmoCells = 57,
    Items = 58,
    TakeDamage = 59,
    DeadFlag = 61,
    ViewOffsetX = 62,
    ViewOffsetY = 63,
    ViewOffsetZ = 64,
    Button0 = 65,
    Button1 = 66,
    Button2 = 67,
    Impulse = 68,
    FixAngle = 69,
    ViewAngleX = 70,
    ViewAngleY = 71,
    ViewAngleZ = 72,
    IdealPitch = 73,
    Flags = 76,
    Colormap = 77,
    Team = 78,
    MaxHealth = 79,
    TeleportTime = 80,
    ArmorStrength = 81,
    ArmorValue = 82,
    WaterLevel = 83,
    Contents = 84,
    IdealYaw = 85,
    YawSpeed = 86,
    SpawnFlags = 89,
    DmgTake = 92,
    DmgSave = 93,
    MoveDirectionX = 96,
    MoveDirectionY = 97,
    MoveDirectionZ = 98,
    Sounds = 100,
}

/* 
impl FieldAddr for FieldAddrFloat {
    type Value = f32;

    #[inline]
    fn load(&self, ent: &Entity) -> Result<Self::Value, EntityError> {
        ent.get_float(*self as i16)
    }

    #[inline]
    fn store(&self, ent: &mut Entity, value: Self::Value) -> Result<(), EntityError> {
        ent.put_float(value, *self as i16)
    }
}
*/


#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum FieldAddrVector {
    AbsMin = 1,
    AbsMax = 4,
    Origin = 10,
    OldOrigin = 13,
    Velocity = 16,
    Angles = 19,
    AngularVelocity = 22,
    PunchAngle = 25,
    Mins = 33,
    Maxs = 36,
    Size = 39,
    ViewOffset = 62,
    ViewAngle = 70,
    MoveDirection = 96,
}

/* 
impl FieldAddr for FieldAddrVector {
    type Value = [f32; 3];

    #[inline]
    fn load(&self, ent: &Entity) -> Result<Self::Value, EntityError> {
        ent.get_vector(*self as i16)
    }

    #[inline]
    fn store(&self, ent: &mut Entity, value: Self::Value) -> Result<(), EntityError> {
        ent.put_vector(value, *self as i16)
    }
}
*/
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum FieldAddrStringId {
    ClassName = 28,
    ModelName = 29,
    WeaponModelName = 51,
    NetName = 74,
    Target = 90,
    TargetName = 91,
    Message = 99,
    Noise0Name = 101,
    Noise1Name = 102,
    Noise2Name = 103,
    Noise3Name = 104,
}
/* 
impl FieldAddr for FieldAddrStringId {
    type Value = StringId;

    fn load(&self, ent: &Entity) -> Result<Self::Value, EntityError> {
        ent.get_int(*self as i16)
            .map(|val| StringId(val.try_into().unwrap()))
    }

    fn store(&self, ent: &mut Entity, value: Self::Value) -> Result<(), EntityError> {
        ent.put_int(value.0.try_into().unwrap(), *self as i16)
    }
}*/

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum FieldAddrEntityId {
    /// The entity this entity is standing on.
    Ground = 47,
    Chain = 60,
    Enemy = 75,
    Aim = 87,
    Goal = 88,
    DmgInflictor = 94,
    Owner = 95,
}

/* 

impl FieldAddr for FieldAddrEntityId {
    type Value = EntityId;

    fn load(&self, ent: &Entity) -> Result<Self::Value, EntityError> {
        ent.entity_id(*self as i16)
    }

    fn store(&self, ent: &mut Entity, value: Self::Value) -> Result<(), EntityError> {
        ent.put_entity_id(value, *self as i16)
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum FieldAddrFunctionId {
    Touch = 42,
    Use = 43,
    Think = 44,
    Blocked = 45,
}

impl FieldAddr for FieldAddrFunctionId {
    type Value = FunctionId;

    #[inline]
    fn load(&self, ent: &Entity) -> Result<Self::Value, EntityError> {
        ent.function_id(*self as i16)
    }

    #[inline]
    fn store(&self, ent: &mut Entity, value: Self::Value) -> Result<(), EntityError> {
        ent.put_function_id(value, *self as i16)
    }
}*/

bitflags! {
    pub struct EntityFlags: u16 {
        const FLY            = 0b0000000000001;
        const SWIM           = 0b0000000000010;
        const CONVEYOR       = 0b0000000000100;
        const CLIENT         = 0b0000000001000;
        const IN_WATER       = 0b0000000010000;
        const MONSTER        = 0b0000000100000;
        const GOD_MODE       = 0b0000001000000;
        const NO_TARGET      = 0b0000010000000;
        const ITEM           = 0b0000100000000;
        const ON_GROUND      = 0b0001000000000;
        const PARTIAL_GROUND = 0b0010000000000;
        const WATER_JUMP     = 0b0100000000000;
        const JUMP_RELEASED  = 0b1000000000000;
    }
}

// TODO: if this never gets used, remove it
#[allow(dead_code)]
fn float_addr(addr: usize) -> Result<FieldAddrFloat, WorldError> {
    match FieldAddrFloat::from_usize(addr) {
        Some(f) => Ok(f),
        None => Err(WorldError::with_msg(format!(
            "float_addr: invalid address ({})",
            addr
        ))),
    }
}

// TODO: if this never gets used, remove it
#[allow(dead_code)]
fn vector_addr(addr: usize) -> Result<FieldAddrVector, WorldError> {
    match FieldAddrVector::from_usize(addr) {
        Some(v) => Ok(v),
        None => Err(WorldError::with_msg(format!(
            "vector_addr: invalid address ({})",
            addr
        ))),
    }
}

 

#[derive(Debug)]
pub struct EntityTypeDef {
    
   /* addr_count: usize,
    field_defs: Box<[FieldDef]>,

    name_cache: RefCell<LRUCache<FieldDefCacheEntry, 16>>,*/ 
}
/*
impl EntityTypeDef {
    pub fn new(
        
        addr_count: usize,
        field_defs: Box<[FieldDef]>,
    ) -> Result<EntityTypeDef, EntityError> {
        if addr_count < STATIC_ADDRESS_COUNT {
            return Err(EntityError::with_msg(format!(
                "addr_count ({}) < STATIC_ADDRESS_COUNT ({})",
                addr_count, STATIC_ADDRESS_COUNT
            )));
        }

        Ok(EntityTypeDef {
             
            addr_count,
            field_defs,
            name_cache: RefCell::new(LRUCache::default()),
        })
    }

    pub fn addr_count(&self) -> usize {
        self.addr_count
    }

    pub fn field_defs(&self) -> &[FieldDef] {
        self.field_defs.as_ref()
    }

    /// Locate a field definition given its name.
    pub fn find<S>(&self, name: S) -> Option<&FieldDef>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();

        if let Some(cached) = self
            .name_cache
            .borrow_mut()
            .find(|entry| &entry.name == name)
        {
            return Some(&self.field_defs[cached.index]);
        }

        let name_id = self.string_table.borrow().find(name)?;

        let (index, def) = self
            .field_defs
            .iter()
            .enumerate()
            .find(|(_, def)| def.name_id == name_id)?;

        self.name_cache.borrow_mut().insert(FieldDefCacheEntry {
            name: ArrayString::from(name).unwrap(),
            index,
        });

        Some(def)
    }
}
*/


#[derive(Debug, FromPrimitive, PartialEq)]
pub enum EntitySolid {
    Not = 0,
    Trigger = 1,
    BBox = 2,
    SlideBox = 3,
    Bsp = 4,
}

#[derive(Debug)]
pub struct Entity {
    
    //type_def: Rc<EntityTypeDef>,
    //addrs: Box<[[u8; 4]]>,

    //components 
    type_def: HashMap<String, String>, 
    //model_index : usize, // consider putting this in a component ? 


    pub leaf_count: usize,
    pub leaf_ids: [usize; MAX_ENT_LEAVES],
    pub baseline: EntityState,
}


// an entity can have  key-value pairs on it from this type_def  (config within trenchbroom!)
// an entity can also have components registered to it with init magnitudes (config within slime!)
// this is the core of our world and levelscene !  This is what makes the game a game . 
impl Entity {
    pub fn new( ) -> Entity {
        let type_def: HashMap<String, String> = HashMap::new();

        Entity {           
            type_def, 
            leaf_count: 0,
            leaf_ids: [0; MAX_ENT_LEAVES],
            baseline: EntityState::uninitialized(),
        }
    }
 

    //maybe add bevy for this? or do it the bevy way 
    pub fn registerComponent(){}    


    /*
    pub fn model_index(&self) -> Result<usize, EntityError> {
        let model_index = self.get_float(FieldAddrFloat::ModelIndex as i16)?;
        if model_index < 0.0 || model_index > ::std::usize::MAX as f32 {
            Err(EntityError::with_msg(format!(
                "Invalid value for entity.model_index ({})",
                model_index,
            )))
        } else {
            Ok(model_index as usize)
        }
    }*/ 


}


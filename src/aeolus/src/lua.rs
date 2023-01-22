use rlua::{Lua, Variadic};
use common::unit::{UnitNum, RefDim};
use common::number::Real;
use grid::block::BlockCollection;

pub fn create_lua_state() -> Lua {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        // register some functions for use in lua
        let globals = lua_ctx.globals();

        // Numbers with units
        let unit_num = lua_ctx.create_function(|_,(value, unit): (Real, String)| {
            Ok(UnitNum::new(value, &unit))
        }).unwrap();
        globals.set("UnitNum", unit_num).unwrap();

        // system of reference dimension
        let ref_dim = lua_ctx.create_function(|_,ref_dims: Variadic<UnitNum>| {
            let ref_dims_vec: Vec<UnitNum> = ref_dims.iter().cloned().collect();
            Ok(RefDim::new(ref_dims_vec))
        }).unwrap();
        globals.set("RefDim", ref_dim).unwrap();

        // Block input
        let block_collection = lua_ctx.create_function(|_,()| {
            Ok(BlockCollection::new())
        }).unwrap();
        globals.set("blocks", block_collection).unwrap();

        // the config table
        let config = lua_ctx.create_table().unwrap();
        globals.set("config", config).unwrap();
    });

    lua
}

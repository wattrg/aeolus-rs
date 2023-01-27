use std::path::PathBuf;
use std::fs::read_to_string;

use rlua::Table;

use common::DynamicResult;
use crate::settings::{AeolusSettings, SimSettings};
use crate::lua::create_lua_state;

pub fn prep_sim(sim: &mut PathBuf, settings: &AeolusSettings) -> DynamicResult<()> {
    settings.file_structure().create_directories();
    let mut sim_settings = SimSettings::default();
    let lua_file = read_to_string(sim)?;
    // set up simulation configuration from the lua script
    let lua = create_lua_state();
    lua.context(|lua_ctx| -> DynamicResult<()> {
        let globals = lua_ctx.globals();

        // execute the lua script
        lua_ctx.load(&lua_file)
            .exec()?;

        // get the config table
        let config = globals.get::<_, Table>("config").unwrap();
        sim_settings = SimSettings::from_lua_table(config).unwrap();

        Ok(())
    })?;

    sim_settings.write_config(settings.file_structure())?; 

    Ok(())
}

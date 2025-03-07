use mlua::{AnyUserData, ExternalError, Function, Lua};
use yazi_proxy::{AppProxy, HIDER};

use super::Utils;
use crate::bindings::{Permit, PermitRef};

impl Utils {
	pub(super) fn hide(lua: &Lua) -> mlua::Result<Function> {
		lua.create_async_function(|lua, ()| async move {
			if lua.named_registry_value::<PermitRef<fn()>>("HIDE_PERMIT").is_ok_and(|h| h.is_some()) {
				return Err("Cannot hide while already hidden".into_lua_err());
			}

			let permit = HIDER.acquire().await.unwrap();
			AppProxy::stop().await;

			lua.set_named_registry_value("HIDE_PERMIT", Permit::new(permit, AppProxy::resume as fn()))?;
			lua.named_registry_value::<AnyUserData>("HIDE_PERMIT")
		})
	}
}

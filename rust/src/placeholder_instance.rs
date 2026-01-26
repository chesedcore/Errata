use godot::prelude::*;
use godot::classes::{Script, ScriptLanguage};
use godot::meta::PropertyInfo;
use godot::obj::script::{ScriptInstance, SiMut} ;

use crate::CoronaScript;
use std::collections::HashMap;


#[derive(Debug)]
pub struct CoronaScriptInstancePlaceholder {
    pub props: HashMap<StringName, Variant>,
    pub script_ref: Gd<Script>,
}

impl CoronaScriptInstancePlaceholder {
    pub fn new(script: Gd<CoronaScript>) -> Self {
        let script_ref = script.clone().upcast::<Script>();
        Self {
            props: HashMap::new(),
            script_ref,
        }
    }
}

impl ScriptInstance for CoronaScriptInstancePlaceholder {
    type Base = Object;

    fn class_name(&self) -> GString {
        "".into()
    }

    fn set_property(mut this: SiMut<'_, Self>, name: StringName, value: &Variant) -> bool {
        this.props.insert(name, value.clone());
        true
    }

    fn get_property(&self, name: StringName) -> Option<Variant> {
        self.props.get(&name).cloned()
    }

    fn get_property_list(&self) -> Vec<PropertyInfo> {
        vec![]
    }

    fn get_method_list(&self) -> Vec<godot::meta::MethodInfo> {
        vec![]
    }

    fn call(mut _this: SiMut<'_, Self>, _method: StringName, _args: &[&Variant]) -> Result<Variant, godot::sys::GDExtensionCallErrorType> {
        Ok(Variant::nil())
    }

    fn is_placeholder(&self) -> bool {
        true
    }

    fn has_method(&self, _: StringName) -> bool {
        false
    }

    fn get_script(&self) -> &Gd<Script> {
        &self.script_ref
    }

    fn get_property_type(&self, _: StringName) -> VariantType {
        VariantType::NIL
    }

    fn to_string(&self) -> GString {
        "".into()
    }

    fn get_property_state(&self) -> Vec<(StringName, Variant)> {
        self.props.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    fn get_language(&self) -> Gd<ScriptLanguage> {
        crate::Corona::singleton().clone()
    }

    fn on_refcount_decremented(&self) -> bool { false }
    fn on_refcount_incremented(&self) {}
    fn property_get_fallback(&self, _name: StringName) -> Option<Variant> { None }
    fn property_set_fallback(_this: SiMut<'_, Self>, _name: StringName, _value: &Variant) -> bool { false }
    fn get_method_argument_count(&self, _method: StringName) -> Option<u32> { None }
}

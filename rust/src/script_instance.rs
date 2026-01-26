use godot::prelude::*;
use godot::classes::{Script, ScriptLanguage};
use godot::meta::{PropertyInfo, MethodInfo};
use godot::obj::script::{ScriptInstance, SiMut};

use crate::ErrataScript;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ErrataScriptInstance {
    pub props: HashMap<StringName, Variant>,
    pub script_ref: Gd<Script>,
    pub owner: Gd<Object>,
}

impl ErrataScriptInstance {
    pub fn new(script: Gd<ErrataScript>, owner: Gd<Object>) -> Self {
        let script_ref = script.clone().upcast::<Script>();
        Self {
            props: HashMap::new(),
            script_ref,
            owner,
        }
    }
}

impl ScriptInstance for ErrataScriptInstance {
    type Base = Object;

    fn class_name(&self) -> GString {
        "ErrataScriptInstance".into()
    }

    fn set_property(mut this: SiMut<'_, Self>, name: StringName, value: &Variant) -> bool {
        // godot_print!("Setting property: {} = {:?}", name, value);
        this.props.insert(name, value.clone());
        true
    }

    fn get_property(&self, name: StringName) -> Option<Variant> {
        // godot_print!("Getting property: {}", name);
        self.props.get(&name).cloned()
    }

    fn get_property_list(&self) -> Vec<PropertyInfo> {
        // godot_print!("Getting property list");
        // I guess I want to return properties we've stored dynamically
        // later, I'll probs to parse the script and return declared properties lmao
        vec![]
    }

    fn get_method_list(&self) -> Vec<MethodInfo> {
        // godot_print!("Getting method list");
        // TODO: Parse script and return actual methods!!!!
        vec![]
    }

    fn call(mut _this: SiMut<'_, Self>, method: StringName, args: &[&Variant]) -> Result<Variant, godot::sys::GDExtensionCallErrorType> {
        // godot_print!("Calling method: {} with {} args", method, args.len());
        
        match method.to_string().as_str() {
            "_ready" => {
                godot_print!("_ready() called on Errata script!");
                // TODO: exec the actual _ready function from the script lol
                Ok(Variant::nil())
            }
            "_process" => {
                // TODO: exec _process(delta)
                Ok(Variant::nil())
            }
            "_physics_process" => {
                // TODO: exec _physics_process(delta)
                Ok(Variant::nil())
            }
            "_input" => {
                // TODO: exec _input(event)
                Ok(Variant::nil())
            }
            "_unhandled_input" => {
                // TODO: exec _unhandled_input(event)
                Ok(Variant::nil())
            }
            _ => {
                godot_print!("Unknown method called: {}", method);
                // TODO: lookup random ahh method then call it
                Ok(Variant::nil())
            }
        }
    }

    fn is_placeholder(&self) -> bool {
        false
    }

    fn has_method(&self, method: StringName) -> bool {
        // godot_print!("Checking if has method: {}", method);
        // TODO: look through the script to see if that method exists
        // but for now, default to common godot callbacks
        matches!(
            method.to_string().as_str(),
            "_ready" | "_process" | "_physics_process" | "_input" | "_unhandled_input"
        )
    }

    fn get_script(&self) -> &Gd<Script> {
        &self.script_ref
    }

    fn get_property_type(&self, name: StringName) -> VariantType {
        // godot_print!("Getting property type for: {}", name);
        // TODO: parse the damned script and return actual property types
        self.props.get(&name)
            .map(|v| v.get_type())
            .unwrap_or(VariantType::NIL)
    }

    fn to_string(&self) -> GString {
        format!("[ErrataScriptInstance:{}]", self.script_ref.get_path()).into()
    }

    fn get_property_state(&self) -> Vec<(StringName, Variant)> {
        // godot_print!("Getting property state");
        self.props.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    fn get_language(&self) -> Gd<ScriptLanguage> {
        crate::Errata::singleton()
    }

    fn on_refcount_decremented(&self) -> bool {
        // godot_print!("Refcount decremented");
        false
    }

    fn on_refcount_incremented(&self) {
        // godot_print!("Refcount incremented");
    }

    fn property_get_fallback(&self, name: StringName) -> Option<Variant> {
        // godot_print!("Property get fallback: {}", name);
        None
    }

    fn property_set_fallback(_this: SiMut<'_, Self>, name: StringName, _value: &Variant) -> bool {
        // godot_print!("Property set fallback: {}", name);
        false
    }

    fn get_method_argument_count(&self, method: StringName) -> Option<u32> {
        // godot_print!("Getting method argument count for: {}", method);
        // TODO: parse the damned script and return actual argument count
        match method.to_string().as_str() {
            "_ready" => Some(0),
            "_process" | "_physics_process" => Some(1),
            "_input" | "_unhandled_input" => Some(1),
            _ => None,
        }
    }
}
use godot::{classes::{IScriptExtension, Script, ScriptExtension, ScriptLanguage}, obj::script::create_script_instance, prelude::*};

use crate::{errata_parse, errata_prelude::ast::Statement, godot_bridge::errata::Errata, godot_bridge::placeholder_instance::ErrataScriptInstancePlaceholder, godot_bridge::script_instance::ErrataScriptInstance};

#[derive(GodotClass)]
#[class(tool, no_init, base=ScriptExtension)]
pub struct ErrataScript {
    pub base: Base<ScriptExtension>,
    pub source_code: GString,
    pub language: Option<Gd<ScriptLanguage>>,
    pub ast: Option<Vec<Statement>>
}

#[godot_api]
impl IScriptExtension for ErrataScript {

    fn init(base: Base<ScriptExtension>) -> Self {
        Self {
            base,
            source_code: GString::new(),
            language: Some(Errata::singleton()),
            ast: None,
        }
    }

    fn editor_can_reload_from_file(&mut self) -> bool { false }
    fn can_instantiate(&self) -> bool { true }
    fn get_base_script(&self) -> Option<Gd<Script>> { None }
    fn get_global_name(&self) -> StringName { StringName::default() }
    fn inherits_script(&self, _script: Gd<Script>) -> bool { false }
    fn get_instance_base_type(&self) -> StringName { 
        StringName::from("RefCounted")
    }
    fn instance_has(&self, _object: Gd<Object>) -> bool { false }
    fn has_source_code(&self) -> bool { true }
    fn get_source_code(&self) -> GString { self.source_code.clone() }

    //parse the ast immediately on script source set
    fn set_source_code(&mut self, code: GString) {

        self.source_code = code.clone();

        let script_parser = errata_parse::ScriptParser::new();
        match script_parser.parse(&code.to_string()) {

            Ok(ast) => { 
                godot_print!("Parsed successfully! AST: {:?}", ast);
                self.ast = Some(ast);
            },

            Err(e) => {
                godot_script_error!("Parse error: {:?}", e);
                godot_error!("Parser error: {:?}", e);
                self.ast = None;
            }
        }
    
    }

    fn reload(&mut self, _keep_state: bool) -> godot::global::Error {
        godot::global::Error::OK
    }
    fn get_documentation(&self) -> Array<Dictionary> { Array::new() }
    fn get_class_icon_path(&self) -> GString { GString::new() }
    fn has_method(&self, _method: StringName) -> bool { false }
    fn has_static_method(&self, _method: StringName) -> bool { false }
    fn get_method_info(&self, _method: StringName) -> Dictionary { Dictionary::new() }
    
    fn is_tool(&self) -> bool { false }
    fn is_valid(&self) -> bool { true }
    fn is_abstract(&self) -> bool { false }
    
    fn get_language(&self) -> Option<Gd<ScriptLanguage>> {
        self.language.clone()
    }
    
    fn has_script_signal(&self, _signal: StringName) -> bool { false }
    fn get_script_signal_list(&self) -> Array<Dictionary> { Array::new() }

    fn has_property_default_value(&self, _property: StringName) -> bool { false }
    fn get_property_default_value(&self, _property: StringName) -> Variant { Variant::nil() }

    fn update_exports(&mut self) {}

    fn get_script_method_list(&self) -> Array<Dictionary> { Array::new() }
    fn get_script_property_list(&self) -> Array<Dictionary> { Array::new() }

    fn get_member_line(&self, _member: StringName) -> i32 { -1 }

    fn get_constants(&self) -> Dictionary { Dictionary::new() }
    fn get_members(&self) -> Array<StringName> { Array::new() }

    fn is_placeholder_fallback_enabled(&self) -> bool {
        // godot_print!("Placeholder fallback triggered!");
        true 
    }
    fn get_rpc_config(&self) -> Variant { Variant::nil() }
    
    //unsafe!
    unsafe fn instance_create(&self, for_object: Gd<Object>) -> *mut std::ffi::c_void {
        godot_print!("Creating ErrataScriptInstance for object: {:?}", for_object);
        let instance = ErrataScriptInstance::new(self.to_gd(), for_object.clone());
        create_script_instance(instance, for_object)
    }

    unsafe fn placeholder_instance_create(&self, for_object: Gd<godot::classes::Object>) -> *mut std::ffi::c_void {
        godot_print!("Creating placeholder instance for object: {:?}", for_object);
        let placeholder = ErrataScriptInstancePlaceholder::new(self.to_gd());
        create_script_instance(placeholder, for_object)
    }
}
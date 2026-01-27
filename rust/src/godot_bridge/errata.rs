use godot::{classes::{IScriptLanguageExtension, Script, ScriptLanguage, ScriptLanguageExtension, native::ScriptLanguageExtensionProfilingInfo, script_language::ScriptNameCasing}, prelude::*};

use crate::godot_bridge::errata_script::ErrataScript;


#[derive(GodotClass)]
#[class(tool, init, base=ScriptLanguageExtension)]
pub struct Errata {
    base: Base<ScriptLanguageExtension>,
}

impl Errata {
    pub fn singleton() -> Gd<ScriptLanguage> {
        crate::ERRATA_LANG.with(|cell| unsafe {
            let lang_ref = (*cell.as_ptr()).assume_init_ref();
            lang_ref.clone()
        })
    }
}

#[godot_api]
impl IScriptLanguageExtension for Errata {
    fn get_name(&self) -> GString { "Errata".into() }
    fn get_type(&self) -> GString { "Errata".into() }

    fn init_ext(&mut self) {}
    
    //so from what I see, this wants the file extension of your scripting language. without the dot.
    //like gdscript is ".gd", but without the dot.
    fn get_extension(&self) -> GString { "err".into() }

    //see `get_extension`
    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("err")])
    }
    
    fn finish(&mut self) {}
    fn frame(&mut self) {}
    
    // syntax highlighting helpers
    fn get_reserved_words(&self) -> PackedStringArray { PackedStringArray::new() }
    fn is_control_flow_keyword(&self, _keyword: GString) -> bool { false }
    fn get_comment_delimiters(&self) -> PackedStringArray { PackedStringArray::new() }
    fn get_string_delimiters(&self) -> PackedStringArray { PackedStringArray::new() }
    
    // script creation
    fn make_template(&self, template: GString, _class_name: GString, _base_class: GString) -> Option<Gd<Script>> {
        // godot_print!("MAKE_TEMPLATE CALLED! template={}, class={}, base={}", template, class_name, base_class);
        
        let source = if template.is_empty() {
            GString::from("# Errata Script\n")
        } else {
            template
        };
        
        let script = Gd::from_init_fn(|base| ErrataScript {
            base,
            source_code: source,
            language: Some(Errata::singleton()),
            ast: None,
        });
        
        godot_print!("Template created successfully!");
        Some(script.upcast())
    }


    fn get_built_in_templates(&self, object: StringName) -> Array<Dictionary> {
        match object.to_string().as_str() {
            "Node" => array![&dict! {
                "inherit": StringName::from("Node"),
                "name": "Empty",
                "description": "An empty Errata script.",
                "content": "# Errata Script\nfunc _ready():\n\tpass\n",
                "id": 1,
                "origin": "builtin",
            }],
            _ => Array::new(),
        }
    }

    fn is_using_templates(&mut self) -> bool { true }

    fn create_script(&self) -> Option<Gd<Object>> {
        Some(Gd::from_init_fn(|base| ErrataScript {
            base,
            source_code: GString::new(),
            language: Some(Errata::singleton()),
            ast: None,
        }).upcast())
    }
    
    // extra capabilities
    fn has_named_classes(&self) -> bool { false }
    fn supports_builtin_mode(&self) -> bool { true }
    fn supports_documentation(&self) -> bool { false }
    fn can_inherit_from_file(&self) -> bool { false }
    fn can_make_function(&self) -> bool { false }
    fn overrides_external_editor(&mut self) -> bool { false }
    
    // code editing features
    fn validate(&self, _script: GString, _: GString, _: bool, _: bool, _: bool, _: bool) -> Dictionary { 
        // godot_print!("VALIDATING: {}", script);
        let mut result = Dictionary::new();
        result.set("valid", true);
        result.set("errors", Array::<Dictionary>::new());
        result.set("warnings", Array::<Dictionary>::new());
        result
    }

    fn validate_path(&self, _path: GString) -> GString { GString::new() }

    fn find_function(&self, _: GString, _: GString) -> i32 { -1 }
    fn make_function(&self, _: GString, _: GString, _: PackedStringArray) -> GString { GString::new() }

    fn open_in_external_editor(&mut self, _: Option<Gd<Script>>, _: i32, _: i32) -> godot::global::Error { 
        godot::global::Error::ERR_UNAVAILABLE 
    }

    fn preferred_file_name_casing(&self) -> ScriptNameCasing {
        ScriptNameCasing::SNAKE_CASE
    }

    fn complete_code(&self, _: GString, _: GString, _: Option<Gd<Object>>) -> Dictionary { Dictionary::new() }
    fn lookup_code(&self, _: GString, _: GString, _: GString, _: Option<Gd<Object>>) -> Dictionary { Dictionary::new() }
    fn auto_indent_code(&self, code: GString, _: i32, _: i32) -> GString { code }
    
    // globals
    fn add_global_constant(&mut self, _: StringName, _: Variant) {}
    fn add_named_global_constant(&mut self, _: StringName, _: Variant) {}
    fn remove_named_global_constant(&mut self, _: StringName) {}
    
    // threading
    fn thread_enter(&mut self) {}
    fn thread_exit(&mut self) {}
    
    // debugging
    fn debug_get_error(&self) -> GString { GString::new() }
    fn debug_get_stack_level_count(&self) -> i32 { 0 }
    fn debug_get_stack_level_line(&self, _: i32) -> i32 { 0 }
    fn debug_get_stack_level_function(&self, _: i32) -> GString { GString::new() }
    fn debug_get_stack_level_source(&self, _: i32) -> GString { GString::new() }
    fn debug_get_stack_level_locals(&mut self, _: i32, _: i32, _: i32) -> Dictionary { Dictionary::new() }
    fn debug_get_stack_level_members(&mut self, _: i32, _: i32, _: i32) -> Dictionary { Dictionary::new() }
    fn debug_get_globals(&mut self, _: i32, _: i32) -> Dictionary { Dictionary::new() }
    fn debug_parse_stack_level_expression(&mut self, _: i32, _: GString, _: i32, _: i32) -> GString { GString::new() }
    fn debug_get_current_stack_info(&mut self) -> Array<Dictionary> { Array::new() }

    // script reloads
    fn reload_all_scripts(&mut self) {}
    fn reload_tool_script(&mut self, _: Option<Gd<Script>>, _: bool) {}
    
    // public api
    fn get_public_functions(&self) -> Array<Dictionary> { Array::new() }
    fn get_public_constants(&self) -> Dictionary { Dictionary::new() }
    fn get_public_annotations(&self) -> Array<Dictionary> { Array::new() }
    
    // skip profiling ig
    fn profiling_start(&mut self) {}
    fn profiling_stop(&mut self) {}
    fn profiling_set_save_native_calls(&mut self, _: bool) {}
    
    // class handling
    fn handles_global_class_type(&self, _: GString) -> bool { false }
    fn get_global_class_name(&self, _: GString) -> Dictionary { Dictionary::new() }
    
    //unsafe
    unsafe fn debug_get_stack_level_instance(&mut self, _: i32) -> *mut std::ffi::c_void { std::ptr::null_mut() }
    unsafe fn profiling_get_accumulated_data(&mut self, _: *mut ScriptLanguageExtensionProfilingInfo, _: i32) -> i32 { 0 }
    unsafe fn profiling_get_frame_data(&mut self, _: *mut ScriptLanguageExtensionProfilingInfo, _: i32) -> i32 { 0 }
}
mod placeholder_instance;

use godot::{classes::{Engine, FileAccess, IResourceFormatLoader, IResourceFormatSaver, IScriptExtension, IScriptLanguageExtension, Resource, ResourceFormatLoader, ResourceFormatSaver, ResourceLoader, ResourceSaver, Script, ScriptExtension, ScriptLanguage, ScriptLanguageExtension}, obj::script::create_script_instance, prelude::*};
use godot::classes::script_language::ScriptNameCasing;
use godot::classes::native::ScriptLanguageExtensionProfilingInfo;
use godot::classes::file_access::ModeFlags;
use std::{cell::Cell, mem::MaybeUninit};
use crate::placeholder_instance::CoronaScriptInstancePlaceholder;

struct MyExtension;

thread_local! {
    static CORONA_LANG: Cell<MaybeUninit<Gd<Corona>>> =
        const { Cell::new(MaybeUninit::uninit()) };
    static CORONA_LOADER: Cell<MaybeUninit<Gd<CoronaResourceLoader>>> =
        const { Cell::new(MaybeUninit::uninit()) };
    static CORONA_SAVER: Cell<MaybeUninit<Gd<CoronaResourceSaver>>> =
        const { Cell::new(MaybeUninit::uninit()) };
}

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let corona = Corona::new_alloc();
            
            Engine::singleton().register_script_language(&corona);
            Engine::singleton().register_singleton(&Corona::class_name().to_string_name(), &corona);
            CORONA_LANG.set(MaybeUninit::new(corona));

            //register loader and saver
            let loader = Gd::from_object(CoronaResourceLoader);
            ResourceLoader::singleton().add_resource_format_loader(&loader);
            CORONA_LOADER.set(MaybeUninit::new(loader));

            let saver = Gd::from_object(CoronaResourceSaver);
            ResourceSaver::singleton().add_resource_format_saver(&saver);
            CORONA_SAVER.set(MaybeUninit::new(saver));
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            CORONA_LANG.with(|cell| {
                let lang = cell.replace(MaybeUninit::uninit());
                let lang = unsafe { lang.assume_init() };
                Engine::singleton().unregister_script_language(&lang);
                Engine::singleton().unregister_singleton(&Corona::class_name().to_string_name());
                lang.free();
            });

            CORONA_LOADER.with(|cell| {
                let loader = cell.replace(MaybeUninit::uninit());
                let loader = unsafe { loader.assume_init() };
                ResourceLoader::singleton().remove_resource_format_loader(&loader);
            });

            CORONA_SAVER.with(|cell| {
                let saver = cell.replace(MaybeUninit::uninit());
                let saver = unsafe { saver.assume_init() };
                ResourceSaver::singleton().remove_resource_format_saver(&saver);
            });
        }
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=ScriptLanguageExtension)]
struct Corona {
    base: Base<ScriptLanguageExtension>,
}

impl Corona {
    fn singleton() -> Gd<ScriptLanguage> {
        CORONA_LANG.with(|cell| unsafe {
            let lang_ref = (*cell.as_ptr()).assume_init_ref();
            lang_ref.clone().upcast()
        })
    }
}

#[godot_api]
impl IScriptLanguageExtension for Corona {
    fn get_name(&self) -> GString { "Corona".into() }
    fn get_type(&self) -> GString { "Corona".into() }

    fn init_ext(&mut self) {}
    
    //so from what I see, this wants the file extension of your scripting language. without the dot.
    //like gdscript is ".gd", but without the dot.
    fn get_extension(&self) -> GString { "crn".into() }

    //see `get_extension`
    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("crn")])
    }
    
    fn finish(&mut self) {}
    fn frame(&mut self) {}
    
    // syntax highlighting helpers
    fn get_reserved_words(&self) -> PackedStringArray { PackedStringArray::new() }
    fn is_control_flow_keyword(&self, _keyword: GString) -> bool { false }
    fn get_comment_delimiters(&self) -> PackedStringArray { PackedStringArray::new() }
    fn get_string_delimiters(&self) -> PackedStringArray { PackedStringArray::new() }
    
    // script creation
    fn make_template(&self, _template: GString, _class_name: GString, _base_class: GString) -> Option<Gd<Script>> {
        let script = Gd::from_init_fn(|base| CoronaScript {
            base,
            source_code: GString::from("# Corona Script\n"),
        });
        
        Some(script.upcast())
    }


    fn get_built_in_templates(&self, _: StringName) -> Array<Dictionary> { Array::new() }
    fn is_using_templates(&mut self) -> bool { true }

    fn create_script(&self) -> Option<Gd<Object>> {
        Some(Gd::from_init_fn(|base| CoronaScript {
            base,
            source_code: GString::new(),
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
    fn validate(&self, script: GString, _: GString, _: bool, _: bool, _: bool, _: bool) -> Dictionary { 
        godot_print!("VALIDATING: {}", script);
        let mut result = Dictionary::new();
        result.set("valid", true);
        result.set("errors", Array::<Dictionary>::new());
        result.set("warnings", Array::<Dictionary>::new());
        result
    }

    fn validate_path(&self, path: GString) -> GString { path }
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

#[derive(GodotClass)]
#[class(tool, no_init, base=ScriptExtension)]
struct CoronaScript {
    base: Base<ScriptExtension>,
    source_code: GString,
}

#[godot_api]
impl IScriptExtension for CoronaScript {

    fn init(base: Base<ScriptExtension>) -> Self {
        Self {
            base,
            source_code: GString::new(),
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
    fn set_source_code(&mut self, code: GString) { 
        self.source_code = code;
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
        Some(Corona::singleton())
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
        godot_print!("Placeholder fallback triggered!");
        true 
    }
    fn get_rpc_config(&self) -> Variant { Variant::nil() }
    
    //unsafe!
    unsafe fn instance_create(&self, _for_object: Gd<Object>) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    unsafe fn placeholder_instance_create(&self, for_object: Gd<godot::classes::Object>) -> *mut std::ffi::c_void {
        let placeholder = CoronaScriptInstancePlaceholder::new(self.to_gd());
        create_script_instance(placeholder, for_object)
    }
}

#[derive(GodotClass)]
#[class(tool, base=ResourceFormatLoader)]
struct CoronaResourceLoader;

#[godot_api]
impl IResourceFormatLoader for CoronaResourceLoader {
    fn init(_base: Base<ResourceFormatLoader>) -> Self {
        Self
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("crn")])
    }

    fn handles_type(&self, type_name: StringName) -> bool {
        type_name == StringName::from("Script") || type_name == StringName::from("CoronaScript")
    }

    fn get_resource_type(&self, path: GString) -> GString {
        if path.to_string().ends_with(".crn") {
            GString::from("Script")
        } else {
            GString::new()
        }
    }

    fn load(&self, path: GString, _original_path: GString, _use_sub_threads: bool, _cache_mode: i32) -> Variant {
        let file = match FileAccess::open(&path, ModeFlags::READ) {
            Some(f) => f,
            None => {
                godot_error!("Failed to open file: {}", path);
                return Variant::nil();
            }
        };
        
        let source = file.get_as_text();
        
        let mut script = Gd::from_init_fn(|base| CoronaScript {
            base,
            source_code: source,
        });
        
        script.set_path(&path);
        script.to_variant()
    }
}

#[derive(GodotClass)]
#[class(tool, base=ResourceFormatSaver)]
struct CoronaResourceSaver;

#[godot_api]
impl IResourceFormatSaver for CoronaResourceSaver {
    fn init(_base: Base<ResourceFormatSaver>) -> Self {
        Self
    }

    fn get_recognized_extensions(&self, _resource: Option<Gd<Resource>>) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("crn")])
    }

    fn recognize(&self, resource: Option<Gd<Resource>>) -> bool {
        let resource = match resource {
            Some(thing) => thing,
            None => {return false;}
        };
        resource.is_class("Script")
    }

    fn save(&mut self, resource: Option<Gd<Resource>>, path: GString, flags: u32) -> godot::global::Error {
        let Some(resource) = resource else {
            godot_error!("Tried to save null resource!");
            return godot::global::Error::FAILED;
        };
        
        let mut script = resource.cast::<Script>();
        
        use godot::classes::resource_saver::SaverFlags;
        if flags as u64 & SaverFlags::CHANGE_PATH.ord() > 0 {
            script.set_path(&path);
        }
        
        let Some(mut file) = FileAccess::open(&path, ModeFlags::WRITE) else {
            godot_error!("Failed to open file for writing: {}", path);
            return godot::global::Error::FAILED;
        };
        
        file.store_string(&script.get_source_code());
        file.close();
        godot::global::Error::OK
    }
}
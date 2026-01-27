use godot::{classes::{FileAccess, IResourceFormatLoader, IResourceFormatSaver, ResourceFormatLoader, ResourceFormatSaver, Script, file_access::ModeFlags}, prelude::*};

use crate::godot_bridge::{errata::Errata, errata_script::ErrataScript};

#[derive(GodotClass)]
#[class(tool, base=ResourceFormatLoader)]
pub struct ErrataResourceLoader;

#[godot_api]
impl IResourceFormatLoader for ErrataResourceLoader {
    fn init(_base: Base<ResourceFormatLoader>) -> Self { Self }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("err")])
    }

    fn handles_type(&self, type_name: StringName) -> bool {
        type_name == StringName::from("Script") || type_name == StringName::from("ErrataScript")
    }

    fn get_resource_type(&self, path: GString) -> GString {
        if path.to_string().ends_with(".err") {
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
        
        let mut script = Gd::from_init_fn(|base| ErrataScript {
            base,
            source_code: source,
            language: Some(Errata::singleton()),
            ast: None,
        });
        
        script.set_path(&path);
        script.to_variant()
    }
}

#[derive(GodotClass)]
#[class(tool, base=ResourceFormatSaver)]
pub struct ErrataResourceSaver;

#[godot_api]
impl IResourceFormatSaver for ErrataResourceSaver {
    fn init(_base: Base<ResourceFormatSaver>) -> Self {
        Self
    }

    fn get_recognized_extensions(&self, _resource: Option<Gd<Resource>>) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("err")])
    }

    fn recognize(&self, resource: Option<Gd<Resource>>) -> bool {
        resource
            .map(|res| res.try_cast::<ErrataScript>().is_ok())
            .unwrap_or(false)
    }

    fn recognize_path(&self, _res: Option<Gd<Resource>>, _path: GString) -> bool {
        true
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
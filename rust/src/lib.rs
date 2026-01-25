use godot::{classes::{IScriptLanguageExtension, ScriptLanguageExtension}, prelude::*};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(tool, init, base=ScriptLanguageExtension)]
struct Corona {
    base: Base<ScriptLanguageExtension>,
}

#[godot_api]
impl IScriptLanguageExtension for Corona {
    fn get_name(&self) -> GString { "Corona".into() }

    fn get_type(&self) -> GString { "Corona".into() }

    fn init_ext(&mut self) {}

    
}
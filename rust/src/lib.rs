mod errata_prelude;
mod godot_bridge;

use godot::{classes::{Engine, ResourceLoader, ResourceSaver, ScriptLanguage}, prelude::*};
use lalrpop_util::lalrpop_mod;
use std::{cell::Cell, mem::MaybeUninit};
use crate::godot_bridge::{errata::Errata, errata_saveload::{ErrataResourceLoader, ErrataResourceSaver}, errata_script::ErrataScript};


lalrpop_mod!(pub errata_parse);

struct MyExtension;

thread_local! {
    static ERRATA_LANG: Cell<MaybeUninit<Gd<ScriptLanguage>>> =
        const { Cell::new(MaybeUninit::uninit()) };
    static ERRATA_LOADER: Cell<MaybeUninit<Gd<ErrataResourceLoader>>> =
        const { Cell::new(MaybeUninit::uninit()) };
    static ERRATA_SAVER: Cell<MaybeUninit<Gd<ErrataResourceSaver>>> =
        const { Cell::new(MaybeUninit::uninit()) };
}

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let errata = Errata::new_alloc();
            
            Engine::singleton().register_script_language(&errata);
            Engine::singleton().register_singleton(&Errata::class_name().to_string_name(), &errata);
            ERRATA_LANG.set(MaybeUninit::new(errata.upcast::<ScriptLanguage>()));

            //register loader and saver
            let loader = Gd::from_object(ErrataResourceLoader);
            ResourceLoader::singleton().add_resource_format_loader(&loader);
            ERRATA_LOADER.set(MaybeUninit::new(loader));

            let saver = Gd::from_object(ErrataResourceSaver);
            ResourceSaver::singleton().add_resource_format_saver(&saver);
            ERRATA_SAVER.set(MaybeUninit::new(saver));
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            ERRATA_LANG.with(|cell| {
                let lang = cell.replace(MaybeUninit::uninit());
                let lang = unsafe { lang.assume_init() };
                Engine::singleton().unregister_script_language(&lang);
                Engine::singleton().unregister_singleton(&Errata::class_name().to_string_name());
                lang.free();
            });

            ERRATA_LOADER.with(|cell| {
                let loader = cell.replace(MaybeUninit::uninit());
                let loader = unsafe { loader.assume_init() };
                ResourceLoader::singleton().remove_resource_format_loader(&loader);
            });

            ERRATA_SAVER.with(|cell| {
                let saver = cell.replace(MaybeUninit::uninit());
                let saver = unsafe { saver.assume_init() };
                ResourceSaver::singleton().remove_resource_format_saver(&saver);
            });
        }
    }
}

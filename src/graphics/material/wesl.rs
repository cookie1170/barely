use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::prelude::*;

#[macro_export]
macro_rules! wesl_shaders {
    ($($package:expr => $path:expr)+) => {
        $(
            ::barely::graphics::material::Shader::register_wesl_source(
                $package.parse().unwrap(),
                include_str!($path)
            );
        )+
    };
}

static RESOLVER: LazyLock<Mutex<Resolver>> = LazyLock::new(Default::default);

#[derive(Default)]
pub struct Resolver {
    pub pkg: wesl::PkgResolver,
    pub modules: HashMap<ModulePath, &'static str>,
}

impl Resolver {
    /// Gets the resolver instance
    ///
    /// # Panics
    /// If the resolver's mutex is poisoned
    pub fn get() -> MutexGuard<'static, Self> {
        RESOLVER.lock().unwrap()
    }
}

impl wesl::Resolver for Resolver {
    fn resolve_source<'a>(&'a self, path: &ModulePath) -> Result<Cow<'a, str>, wesl::ResolveError> {
        if path.origin.is_package() {
            return self.pkg.resolve_source(path);
        }

        let module = self.modules.get(path);

        let Some(module) = module else {
            let msg =
                "Did you forget to register it with `Shader::register_wesl_source`?".to_string();
            return Err(wesl::ResolveError::ModuleNotFound(path.clone(), msg));
        };

        Ok(Cow::Borrowed(module))
    }
}

impl Shader {
    #[must_use]
    pub fn wesl(module_path: &ModulePath) -> Self {
        Self {
            source: ShaderSource::Single(Self::compile_wesl(module_path)),
            vertex: None,
            fragment: None,
        }
    }

    /// Compiles a WESL shader from its path
    ///
    /// # Panics
    /// When the WESL shader is invalid
    #[must_use]
    pub fn compile_wesl(module_path: &ModulePath) -> wgpu::ShaderSource<'static> {
        let resolver = Resolver::get();
        let mut compiler = wesl::Wesl::new(".").set_custom_resolver(&*resolver);
        if cfg!(target_arch = "wasm32") {
            compiler.set_feature("sixteen_byte_align", true);
        }

        let wgsl = compiler
            .compile(module_path)
            .inspect_err(|e| error!("Failed to compile WESL shader: {e}"))
            .unwrap()
            .to_string();

        wgpu::ShaderSource::Wgsl(wgsl.into())
    }

    pub fn register_wesl_source(path: ModulePath, src: &'static str) {
        let mut resolver = Resolver::get();
        resolver.modules.insert(path, src);
    }
}

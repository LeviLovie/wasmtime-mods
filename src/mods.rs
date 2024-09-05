use wasmtime::*;

pub struct Mod {
    name: String,
    description: String,
    version: String,
    path: String,
    debug_name: String,

    wat_source_path: String,
}

impl Mod {
    pub fn new(relative_path: String) -> Self {
        let path = format!(
            "{}/{}",
            std::env::current_exe()
                .unwrap()
                .as_path()
                .parent()
                .unwrap()
                .to_str()
                .unwrap(),
            relative_path
        );

        let f = match std::fs::File::open(format!("{}/package.yaml", path)) {
            Ok(f) => f,
            Err(e) => {
                error!(
                    "Failed to open the \"package.yaml\" file for the mod {}: {}",
                    path, e
                );
                std::process::exit(1);
            }
        };

        let data: serde_yaml::Value = match serde_yaml::from_reader(f) {
            Ok(d) => d,
            Err(e) => {
                error!(
                    "Failed to parse the \"package.yaml\" file for the mod {}: {}",
                    path, e
                );
                std::process::exit(1);
            }
        };

        let name = match data["name"].as_str() {
            Some(n) => String::from(n),
            None => {
                error!(
                    "The \"name\" field is missing from the \"package.yaml\" file for the mod {}",
                    path
                );
                std::process::exit(1);
            }
        };

        let description = match data["description"].as_str() {
            Some(d) => String::from(d),
            None => {
                error!(
                    "The \"description\" field is missing from the \"package.yaml\" file for the mod {}",
                    path
                );
                std::process::exit(1);
            }
        };

        let version = match data["version"].as_str() {
            Some(v) => String::from(v),
            None => {
                error!(
                    "The \"version\" field is missing from the \"package.yaml\" file for the mod {}",
                    path
                );
                std::process::exit(1);
            }
        };

        let debug_name = format!("{} v{}", name, version);

        let precompiled = match data["precompiled"].as_bool() {
            Some(p) => p,
            None => {
                error!(
                    "The \"precompiled\" field is missing from the \"package.yaml\" file for the mod {}",
                    path
                );
                std::process::exit(1);
            }
        };

        if !precompiled {
            unimplemented!("Precompiled mods are not supported yet");
        }

        let run_relative_path = match data["run"].as_str() {
            Some(r) => String::from(r),
            None => {
                error!(
                    "The \"run\" field is missing from the \"package.yaml\" file for the mod {}",
                    path
                );
                std::process::exit(1);
            }
        };

        let wat_absolute_path = format!("{}{}", path, run_relative_path);

        info!("Wat path: {}", wat_absolute_path);

        return Self {
            name,
            description,
            version,
            path,
            debug_name,

            wat_source_path: wat_absolute_path,
        };
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn debug_name(&self) -> &String {
        &self.debug_name
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn wat_source_path(&self) -> &String {
        &self.wat_source_path
    }

    pub fn init(&self) {
        info!("Initializing mod: {}", self.debug_name);

        let engine = Engine::default();

        let wat = match std::fs::read_to_string(&self.wat_source_path) {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to read the wat file: {}", e);
                std::process::exit(1);
            }
        };
        let module = match Module::new(&engine, wat) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to compile the wat file: {}", e);
                std::process::exit(1);
            }
        };

        let mut linker = Linker::new(&engine);
        match linker.func_wrap(
            "host",
            "host_func",
            |caller: Caller<'_, u32>, param: i32| {
                println!("Got {} from WebAssembly", param);
                println!("my host state is: {}", caller.data());
            },
        ) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to wrap the host function: {}", e);
                std::process::exit(1);
            }
        };

        let mut store: Store<u32> = Store::new(&engine, 4);

        let instance = match linker.instantiate(&mut store, &module) {
            Ok(i) => i,
            Err(e) => {
                error!("Failed to instantiate the module: {}", e);
                std::process::exit(1);
            }
        };
        let hello = match instance.get_typed_func::<(), ()>(&mut store, "hello") {
            Ok(h) => h,
            Err(e) => {
                error!("Failed to get the hello function: {}", e);
                std::process::exit(1);
            }
        };

        match hello.call(&mut store, ()) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to call the hello function: {}", e);
                std::process::exit(1);
            }
        };
    }
}

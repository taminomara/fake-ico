use std::collections::HashMap;

pub enum Cli<'s> {
    Router(Router<'s>),
    Command(Command<'s>),
}

impl Cli<'_> {
    /// Get name of this command.
    pub fn get_name(&self) -> &str {
        match self {
            Cli::Router(r) => r.get_name(),
            Cli::Command(c) => c.get_name(),
        }
    }

    /// Parse program arguments and invoke an appropriate subcommand.
    pub fn run(&self) -> clap::Result<()> {
        self.call(&self.build_clap_app().get_matches())
    }

    /// Invoke an appropriate subcommand based on the given parsed arguments.
    pub fn call(&self, matches: &clap::ArgMatches) -> clap::Result<()> {
        match self {
            Cli::Router(r) => r.call(matches),
            Cli::Command(c) => c.call(matches),
        }
    }

    /// Build [`clap::App`] based on this command.
    pub fn build_clap_app(&self) -> clap::App {
        match self {
            Cli::Router(r) => r.build_clap_app(),
            Cli::Command(c) => c.build_clap_app(),
        }
    }
}

pub struct Command<'s> {
    name: String,
    about: Option<String>,
    long_about: Option<String>,
    args: Vec<clap::Arg<'s, 's>>,
    func: Option<Box<dyn Fn(&clap::ArgMatches) -> clap::Result<()> + 's>>,
}

impl<'s> Command<'s> {
    /// Create a new command or subcommand using the given name to identify it.
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Command {
            name: name.into(),
            about: None,
            long_about: None,
            args: Vec::new(),
            func: None,
        }
    }

    /// Get name of this command.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Add or overwrite a short command's description. See [`clap::App::about`].
    pub fn about<S: Into<String>>(mut self, about: S) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Add or overwrite a full command's description. See [`clap::App::long_about`].
    pub fn long_about<S: Into<String>>(mut self, long_about: S) -> Self {
        self.long_about = Some(long_about.into());
        self
    }

    /// Add an argument for this subcommand.
    pub fn arg(mut self, arg: clap::Arg<'s, 's>) -> Self {
        self.args.push(arg);
        self
    }

    /// Add a callback function that gets invoked when this command is called.
    pub fn func(mut self, func: impl Fn(&clap::ArgMatches) -> clap::Result<()> + 's) -> Self {
        self.func = Some(Box::new(func));
        self
    }

    /// Finish construction of this command and wrap it into [`Cli`].
    pub fn done(self) -> Cli<'s> {
        Cli::Command(self)
    }

    fn call(&self, matches: &clap::ArgMatches) -> clap::Result<()> {
        match &self.func {
            Some(func) => func(matches),
            _ => Ok(()),
        }
    }

    fn build_clap_app(&self) -> clap::App {
        let mut app = clap::App::new(&self.name);

        if let Some(about) = &self.about {
            app = app.about(about.as_str());
        }

        if let Some(long_about) = &self.long_about {
            app = app.long_about(long_about.as_str());
        }

        app = app.args(&self.args);

        app
    }
}

pub struct Router<'s> {
    name: String,
    about: Option<String>,
    long_about: Option<String>,
    args: Vec<clap::Arg<'s, 's>>,
    subcommands: HashMap<String, Cli<'s>>,
}

impl<'s> Router<'s> {
    /// Create a new command or subcommand using the given name to identify it.
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Router {
            name: name.into(),
            about: None,
            long_about: None,
            args: Vec::new(),
            subcommands: HashMap::new(),
        }
    }

    /// Get name of this command.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Add or overwrite a short command's description. See [`clap::App::about`].
    pub fn about<S: Into<String>>(mut self, about: S) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Add or overwrite a full command's description. See [`clap::App::long_about`].
    pub fn long_about<S: Into<String>>(mut self, long_about: S) -> Self {
        self.long_about = Some(long_about.into());
        self
    }

    /// Add an argument for this subcommand.
    pub fn arg(mut self, arg: clap::Arg<'s, 's>) -> Self {
        self.args.push(arg);
        self
    }

    /// Add a subcommand for this command.
    pub fn subcommand(mut self, command: Cli<'s>) -> Self {
        self.subcommands.insert(command.get_name().to_string(), command);
        self
    }

    /// Finish construction of this router and wrap it into [`Cli`].
    pub fn done(self) -> Cli<'s> {
        Cli::Router(self)
    }

    fn call(&self, matches: &clap::ArgMatches) -> clap::Result<()> {
        match matches.subcommand() {
            (name, Some(matches)) => {
                if let Some(subcommand) = self.subcommands.get(name) {
                    subcommand.call(matches)
                } else {
                    Err(clap::Error::with_description(
                        &format!("unknown subcommand {:?}", name),
                        clap::ErrorKind::InvalidSubcommand
                    ))
                }
            },
            (_, None) => {
                Err(clap::Error::with_description(
                    "subcommand is required",
                    clap::ErrorKind::InvalidSubcommand
                ))
            }
        }
    }

    fn build_clap_app(&self) -> clap::App {
        let mut app = clap::App::new(&self.name);

        if let Some(about) = &self.about {
            app = app.about(about.as_str());
        }

        if let Some(long_about) = &self.long_about {
            app = app.long_about(long_about.as_str());
        }

        app = app.args(&self.args);

        for (_, subcommand) in &self.subcommands {
            app = app.subcommand(subcommand.build_clap_app());
        }

        app
    }
}

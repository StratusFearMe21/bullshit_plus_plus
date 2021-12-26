use std::fmt::Display;

use rand_derive2::RandGen;
#[derive(RandGen)]
pub enum Language {
    FrontBack(LanguageRaw, LanguageRaw),
    TwoLangs(LanguageRaw, LanguageRaw),
    Single(LanguageRaw),
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Single(ref lang) => write!(f, "{}", lang),
            Self::TwoLangs(ref lang1, ref lang2) => write!(f, "{} and {}", lang1, lang2),
            Self::FrontBack(ref lang1, ref lang2) => write!(
                f,
                "{} on the front-end, and {} on the back-end",
                lang1, lang2
            ),
        }
    }
}

/// Sourced from Wikipedia
/// https://en.wikipedia.org/wiki/List_of_programming_languages_by_type
/// I only included languages that I knew about

#[derive(RandGen)]
pub enum LanguageRaw {
    Rust,
    Go,
    Python,
    JavaScript(JSVarient),
    Lisp,
    C,
    CPlusPlus,
    Fortran,
    Basic,
    Matlab,
    R,
    WolframLanguage,
    Assembly(ASMVar),
    Ada,
    CSharp(CSharpVar),
    Cobol,
    CommonLisp,
    EmacsLisp,
    VimScript,
    Elm,
    Erlang,
    FSharp,
    Haskell,
    Java,
    Kotlin,
    ObjectiveC,
    Scala,
    Swift,
    OCaml,
    Vala,
    VisualBasic,
    ECMAScript,
    Dart,
    Perl,
    Php,
    PowerShell,
    TypeScript,
    Awk,
    Sql,
    Lua,
    Ruby,
    Scratch,
    Brainfuck,
    Shakespeare,
    Piet,
    Chef,
    OpenCL,
    Pascal,
    AutoHotKey,
    Sed,
    Elixir,
    POSIXShell,
    FishShell,
    BashShell,
    Batch,
}

#[derive(RandGen)]
pub enum JSVarient {
    React,
    AngularJS,
    Angular,
    Node,
    Deno,
    Next,
    Vue,
    Svelte,
    Gatsby,
    Bootstrap,
    Vanilla,
    Express,
    JQuery,
    Three,
}

impl Display for JSVarient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::React => write!(f, "React JS"),
            Self::AngularJS => write!(f, "Angular JS"),
            Self::Angular => write!(f, "Angular"),
            Self::Node => write!(f, "NodeJS"),
            Self::Deno => write!(f, "Deno JS"),
            Self::Next => write!(f, "Next.js"),
            Self::Vue => write!(f, "Vue.js"),
            Self::Svelte => write!(f, "Svelte JS"),
            Self::Gatsby => write!(f, "Gatsby JS"),
            Self::Bootstrap => write!(f, "Bootstrap"),
            Self::Vanilla => write!(f, "Vanilla JavaScript"),
            Self::Express => write!(f, "NodeJS Express"),
            Self::JQuery => write!(f, "jQuery"),
            Self::Three => write!(f, "Three.js"),
        }
    }
}

#[derive(RandGen)]
pub enum ASMVar {
    X86,
    Aarch64,
    I686,
    X8664,
    ArmV7,
    ArmV6,
    ArmV5,
    Mips,
    Mips64,
    Mips64El,
    MipsEl,
    PowerPc,
    PowerPc64,
    PowerPc64Le,
    RiscV,
    S390X,
    Armebv7R,
    ArmV5TE,
    I586,
    ThumbV6M,
    ThumbV7EMEABI,
    ThumbV7EMEABIHF,
    ThumbV7M,
    ThumbV8MBase,
    ThumbV8MMain,
    Wasm,
}

impl Display for ASMVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ASMVar::X86 => write!(f, "x86 Assembly"),
            ASMVar::X8664 => write!(f, "x86_64 Assembly"),
            ASMVar::I686 => write!(f, "i686 Assembly"),
            ASMVar::Wasm => write!(f, "pure WebAssembly"),
            ASMVar::Mips => write!(f, "MIPS Assembly"),
            ASMVar::ThumbV8MMain => write!(f, "ARMv8-M Mainline Assembly"),
            ASMVar::ThumbV8MBase => write!(f, "ARMv8-M Baseline Assembly"),
            ASMVar::ArmV7 => write!(f, "ARMv7 Assembly"),
            ASMVar::ArmV6 => write!(f, "ARMv6 Assembly"),
            ASMVar::ArmV5 => write!(f, "ARMv5 Assembly"),
            ASMVar::ThumbV7M => write!(f, "ARM Cortex M3 Assembly"),
            ASMVar::ThumbV7EMEABI => write!(f, "ARM Cortex M4/M7 Assembly"),
            ASMVar::ThumbV7EMEABIHF => write!(f, "ARM Cortex M4F/M7F Assembly"),
            ASMVar::ArmV5TE => write!(f, "ARMv5TE Assembly"),
            ASMVar::Armebv7R => write!(f, "ARMv7-R Assembly"),
            ASMVar::ThumbV6M => write!(f, "ARM Cortex M0/M1 Assembly"),
            ASMVar::I586 => write!(f, "i586 Assembly"),
            ASMVar::Aarch64 => write!(f, "ARM64 Assembly"),
            ASMVar::MipsEl => write!(f, "MIPS (LE) Assembly"),
            ASMVar::Mips64 => write!(f, "MIPS64 Assembly"),
            ASMVar::Mips64El => write!(f, "MIPS64 (LE) Assembly"),
            ASMVar::RiscV => write!(f, "RISC V Assembly"),
            ASMVar::S390X => write!(f, "S390x Assembly"),
            ASMVar::PowerPc => write!(f, "PowerPC Assembly"),
            ASMVar::PowerPc64 => write!(f, "PPC64 Assembly"),
            ASMVar::PowerPc64Le => write!(f, "PP64LE Assembly"),
        }
    }
}

#[derive(RandGen)]
pub enum CSharpVar {
    ASPDotNet,
    DotNet,
    Mono,
    Unity,
    Uwp,
    Xamarin,
    Blazor,
}

impl Display for CSharpVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ASPDotNet => write!(f, "ASP.NET C#"),
            Self::DotNet => write!(f, ".NET C#"),
            Self::Mono => write!(f, "Mono C#"),
            Self::Unity => write!(f, "Unity C#"),
            Self::Uwp => write!(f, "UWP C#"),
            Self::Xamarin => write!(f, "Xamarin C#"),
            Self::Blazor => write!(f, "Blazor C#"),
        }
    }
}

impl Display for LanguageRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Rust => write!(f, "Rust"),
            Self::Go => write!(f, "Go"),
            Self::Python => write!(f, "Python"),
            Self::JavaScript(ref var) => write!(f, "{}", var),
            Self::Lisp => write!(f, "Lisp"),
            Self::C => write!(f, "C"),
            Self::CPlusPlus => write!(f, "C++"),
            Self::Fortran => write!(f, "Fortran"),
            Self::Basic => write!(f, "BASIC"),
            Self::Matlab => write!(f, "MATLAB"),
            Self::R => write!(f, "R"),
            Self::WolframLanguage => write!(f, "Wolfram Language"),
            Self::Assembly(ref var) => write!(f, "{}", var),
            Self::Ada => write!(f, "Ada"),
            Self::CSharp(ref var) => write!(f, "{}", var),
            Self::Cobol => write!(f, "COBOL"),
            Self::CommonLisp => write!(f, "Common Lisp"),
            Self::EmacsLisp => write!(f, "Emacs Lisp"),
            Self::VimScript => write!(f, "VimScript"),
            Self::Elm => write!(f, "Elm"),
            Self::Erlang => write!(f, "Erlang"),
            Self::FSharp => write!(f, "F#"),
            Self::Haskell => write!(f, "Haskell"),
            Self::Java => write!(f, "Java"),
            Self::Kotlin => write!(f, "Kotlin"),
            Self::ObjectiveC => write!(f, "Objective-C"),
            Self::Scala => write!(f, "Scala"),
            Self::Swift => write!(f, "Swift"),
            Self::OCaml => write!(f, "OCaml"),
            Self::Vala => write!(f, "GNOME Vala"),
            Self::VisualBasic => write!(f, "Visual BASIC"),
            Self::ECMAScript => write!(f, "Vanilla ECMAScript"),
            Self::Dart => write!(f, "Dart"),
            Self::Perl => write!(f, "Perl"),
            Self::Php => write!(f, "PHP"),
            Self::PowerShell => write!(f, "Windows PowerShell"),
            Self::TypeScript => write!(f, "Vanilla TypeScript"),
            Self::Awk => write!(f, "AWK"),
            Self::Sql => write!(f, "SQL"),
            Self::Lua => write!(f, "Lua"),
            Self::Ruby => write!(f, "Ruby"),
            Self::Scratch => write!(f, "Scratch"),
            Self::Brainfuck => write!(f, "Brainfuck"),
            Self::Shakespeare => write!(f, "Shakespeare"),
            Self::Piet => write!(f, "Piet"),
            Self::Chef => write!(f, "Chef"),
            Self::OpenCL => write!(f, "OpenCL"),
            Self::Pascal => write!(f, "Pascal"),
            Self::AutoHotKey => write!(f, "AutoHotKey"),
            Self::Sed => write!(f, "Sed"),
            Self::Elixir => write!(f, "Elixir"),
            Self::POSIXShell => write!(f, "POSIX complient shell"),
            Self::FishShell => write!(f, "fish script"),
            Self::BashShell => write!(f, "bash script"),
            Self::Batch => write!(f, "Windows Batch script"),
        }
    }
}

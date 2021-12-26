use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
};
use rand_derive2::RandGen;
use unchecked_unwrap::UncheckedUnwrap;

#[derive(RandGen)]
pub enum ActionFirst {
    Hyper(&'static ActionSecond),
    Over(&'static ActionSecond),
    Hypo(&'static ActionSecond),
    Mono(&'static ActionSecond),
    Duo(&'static ActionSecond),
    Tri(&'static ActionSecond),
    Hexa(&'static ActionSecond),
    Octa(&'static ActionSecond),
    Octo(&'static ActionSecond),
    Quadro(&'static ActionSecond),
    Demi(&'static ActionSecond),
    Semi(&'static ActionSecond),
    Penta(&'static ActionSecond),
    Meta(&'static ActionSecond),
    NoPrefix(&'static ActionSecond),
}

impl ActionFirst {
    #[inline]
    pub fn ing(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Hyper(a) => write!(f, "hyper{}", a.ing),
            Self::Over(a) => write!(f, "over{}", a.ing),
            Self::Hypo(a) => write!(f, "hypo{}", a.ing),
            Self::Mono(a) => write!(f, "mono{}", a.ing),
            Self::Duo(a) => write!(f, "duo{}", a.ing),
            Self::Tri(a) => write!(f, "tri{}", a.ing),
            Self::Hexa(a) => write!(f, "hexa{}", a.ing),
            Self::Octa(a) => write!(f, "octa{}", a.ing),
            Self::Octo(a) => write!(f, "octo{}", a.ing),
            Self::Quadro(a) => write!(f, "quadro{}", a.ing),
            Self::Demi(a) => write!(f, "demi{}", a.ing),
            Self::Semi(a) => write!(f, "semi{}", a.ing),
            Self::Penta(a) => write!(f, "penta{}", a.ing),
            Self::Meta(a) => write!(f, "meta{}", a.ing),
            Self::NoPrefix(a) => write!(f, "{}", a.ing),
        }
    }
    #[inline]
    pub fn er(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Hyper(a) => write!(f, "hyper{}", a.er),
            Self::Over(a) => write!(f, "over{}", a.er),
            Self::Hypo(a) => write!(f, "hypo{}", a.er),
            Self::Mono(a) => write!(f, "mono{}", a.er),
            Self::Duo(a) => write!(f, "duo{}", a.er),
            Self::Tri(a) => write!(f, "tri{}", a.er),
            Self::Hexa(a) => write!(f, "hexa{}", a.er),
            Self::Octa(a) => write!(f, "octa{}", a.er),
            Self::Octo(a) => write!(f, "octo{}", a.er),
            Self::Quadro(a) => write!(f, "quadro{}", a.er),
            Self::Demi(a) => write!(f, "demi{}", a.er),
            Self::Semi(a) => write!(f, "semi{}", a.er),
            Self::Penta(a) => write!(f, "penta{}", a.er),
            Self::Meta(a) => write!(f, "meta{}", a.er),
            Self::NoPrefix(a) => write!(f, "{}", a.er),
        }
    }
    #[inline]
    pub fn ed(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Hyper(a) => write!(f, "hyper{}", a.ed),
            Self::Over(a) => write!(f, "over{}", a.ed),
            Self::Hypo(a) => write!(f, "hypo{}", a.ed),
            Self::Mono(a) => write!(f, "mono{}", a.ed),
            Self::Duo(a) => write!(f, "duo{}", a.ed),
            Self::Tri(a) => write!(f, "tri{}", a.ed),
            Self::Hexa(a) => write!(f, "hexa{}", a.ed),
            Self::Octa(a) => write!(f, "octa{}", a.ed),
            Self::Octo(a) => write!(f, "octo{}", a.ed),
            Self::Quadro(a) => write!(f, "quadro{}", a.ed),
            Self::Demi(a) => write!(f, "demi{}", a.ed),
            Self::Semi(a) => write!(f, "semi{}", a.ed),
            Self::Penta(a) => write!(f, "penta{}", a.ed),
            Self::Meta(a) => write!(f, "meta{}", a.ed),
            Self::NoPrefix(a) => write!(f, "{}", a.ed),
        }
    }
    #[inline]
    pub fn no_suffix(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Hyper(a) => write!(f, "hyper{}", a.no_suffix),
            Self::Over(a) => write!(f, "over{}", a.no_suffix),
            Self::Hypo(a) => write!(f, "hypo{}", a.no_suffix),
            Self::Mono(a) => write!(f, "mono{}", a.no_suffix),
            Self::Duo(a) => write!(f, "duo{}", a.no_suffix),
            Self::Tri(a) => write!(f, "tri{}", a.no_suffix),
            Self::Hexa(a) => write!(f, "hexa{}", a.no_suffix),
            Self::Octa(a) => write!(f, "octa{}", a.no_suffix),
            Self::Octo(a) => write!(f, "octo{}", a.no_suffix),
            Self::Quadro(a) => write!(f, "quadro{}", a.no_suffix),
            Self::Demi(a) => write!(f, "demi{}", a.no_suffix),
            Self::Semi(a) => write!(f, "semi{}", a.no_suffix),
            Self::Penta(a) => write!(f, "penta{}", a.no_suffix),
            Self::Meta(a) => write!(f, "meta{}", a.no_suffix),
            Self::NoPrefix(a) => write!(f, "{}", a.no_suffix),
        }
    }
}

pub struct ActionSecond {
    pub no_suffix: &'static str,
    pub ing: &'static str,
    pub er: &'static str,
    pub ed: &'static str,
}

impl Distribution<&'static ActionSecond> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> &'static ActionSecond {
        unsafe { ACTIONSECOND.choose(rng).unchecked_unwrap() }
    }
}

pub const ACTIONSECOND: [ActionSecond; 42] = [
    ActionSecond {
        no_suffix: "containerize",
        ing: "containerizing",
        er: "containerizer",
        ed: "containerized",
    },
    ActionSecond {
        no_suffix: "replicate",
        ing: "replicating",
        er: "replicator",
        ed: "replicated",
    },
    ActionSecond {
        no_suffix: "shade",
        ing: "shading",
        er: "shader",
        ed: "shaded",
    },
    ActionSecond {
        no_suffix: "grid",
        ing: "gridding",
        er: "gridder",
        ed: "gridded",
    },
    ActionSecond {
        no_suffix: "code",
        ing: "coding",
        er: "coder",
        ed: "coded",
    },
    ActionSecond {
        no_suffix: "configure",
        ing: "configuring",
        er: "configurer",
        ed: "configured",
    },
    ActionSecond {
        no_suffix: "interface",
        ing: "interfacing",
        er: "interfacer",
        ed: "interfaced",
    },
    ActionSecond {
        no_suffix: "cache",
        ing: "caching",
        er: "cacher",
        ed: "cached",
    },
    ActionSecond {
        no_suffix: "persist",
        ing: "persisting",
        er: "persister",
        ed: "persisted",
    },
    ActionSecond {
        no_suffix: "sign",
        ing: "signing",
        er: "signer",
        ed: "signed",
    },
    ActionSecond {
        no_suffix: "script",
        ing: "scripting",
        er: "scripter",
        ed: "scripted",
    },
    ActionSecond {
        no_suffix: "template",
        ing: "templating",
        er: "templater",
        ed: "templated",
    },
    ActionSecond {
        no_suffix: "poll",
        ing: "polling",
        er: "poller",
        ed: "polled",
    },
    ActionSecond {
        no_suffix: "inject",
        ing: "injecting",
        er: "injecter",
        ed: "injected",
    },
    ActionSecond {
        no_suffix: "pipeline",
        ing: "piplining",
        er: "pipeliner",
        ed: "pipelined",
    },
    ActionSecond {
        no_suffix: "descript",
        ing: "descripting",
        er: "descripter",
        ed: "descripted",
    },
    ActionSecond {
        no_suffix: "vise",
        ing: "vising",
        er: "viser",
        ed: "vised",
    },
    ActionSecond {
        no_suffix: "package",
        ing: "packaging",
        er: "packager",
        ed: "packaged",
    },
    ActionSecond {
        no_suffix: "plug",
        ing: "plugging",
        er: "plugger",
        ed: "plugged",
    },
    ActionSecond {
        no_suffix: "extend",
        ing: "extending",
        er: "extender",
        ed: "extended",
    },
    ActionSecond {
        no_suffix: "modulate",
        ing: "modulating",
        er: "modulator",
        ed: "modulated",
    },
    ActionSecond {
        no_suffix: "host",
        ing: "hosting",
        er: "hoster",
        ed: "hosted",
    },
    ActionSecond {
        no_suffix: "load",
        ing: "loading",
        er: "loader",
        ed: "loaded",
    },
    ActionSecond {
        no_suffix: "store",
        ing: "storing",
        er: "storer",
        ed: "stored",
    },
    ActionSecond {
        no_suffix: "scale",
        ing: "scaling",
        er: "scaler",
        ed: "scaled",
    },
    ActionSecond {
        no_suffix: "certify",
        ing: "certifying",
        er: "certifier",
        ed: "certified",
    },
    ActionSecond {
        no_suffix: "secure",
        ing: "securing",
        er: "securer",
        ed: "secured",
    },
    ActionSecond {
        no_suffix: "virtualize",
        ing: "virtualizing",
        er: "virtualizer",
        ed: "virtualized",
    },
    ActionSecond {
        no_suffix: "optimize",
        ing: "optimizing",
        er: "optimizer",
        ed: "optimized",
    },
    ActionSecond {
        no_suffix: "share",
        ing: "sharing",
        er: "sharer",
        ed: "shared",
    },
    ActionSecond {
        no_suffix: "layer",
        ing: "layering",
        er: "layerer",
        ed: "layered",
    },
    ActionSecond {
        no_suffix: "locate",
        ing: "locating",
        er: "locator",
        ed: "located",
    },
    ActionSecond {
        no_suffix: "allocate",
        ing: "allocating",
        er: "allocator",
        ed: "allocated",
    },
    ActionSecond {
        no_suffix: "flow",
        ing: "flowing",
        er: "flower",
        ed: "flowed",
    },
    ActionSecond {
        no_suffix: "encrypt",
        ing: "encrypting",
        er: "encrypter",
        ed: "encrypted",
    },
    ActionSecond {
        no_suffix: "bloat",
        ing: "bloating",
        er: "bloater",
        ed: "bloated",
    },
    ActionSecond {
        no_suffix: "schedule",
        ing: "scheduling",
        er: "scheduler",
        ed: "scheduled",
    },
    ActionSecond {
        no_suffix: "base",
        ing: "basing",
        er: "baser",
        ed: "based",
    },
    ActionSecond {
        no_suffix: "recurse",
        ing: "recursing",
        er: "recurser",
        ed: "recursed",
    },
    ActionSecond {
        no_suffix: "fix",
        ing: "fixing",
        er: "fixer",
        ed: "fixed",
    },
    ActionSecond {
        no_suffix: "branch",
        ing: "branching",
        er: "brancher",
        ed: "branched",
    },
    ActionSecond {
        no_suffix: "partition",
        ing: "partitioning",
        er: "partitioner",
        ed: "partitioned",
    },
];

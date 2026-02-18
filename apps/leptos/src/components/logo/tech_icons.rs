use leptos::*;

#[derive(Clone, Debug)]
pub struct TechIcon {
    pub name: &'static str,
    pub image: &'static str,
    pub color: &'static str,
}

pub const TECH_STACK: [TechIcon; 6] = [
    TechIcon {
        name: "TypeScript",
        image: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/typescript/typescript-original.svg",
        color: "from-blue-500 to-blue-600",
    },
    TechIcon {
        name: "Rust",
        image: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        color: "from-orange-500 to-red-600",
    },
    TechIcon {
        name: "Kotlin",
        image: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/kotlin/kotlin-original.svg",
        color: "from-purple-500 to-violet-600",
    },
    TechIcon {
        name: "JavaScript",
        image: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/javascript/javascript-original.svg",
        color: "from-yellow-400 to-yellow-600",
    },
    TechIcon {
        name: "Java",
        image: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/java/java-original.svg",
        color: "from-red-500 to-orange-500",
    },
    TechIcon {
        name: "Python",
        image: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg",
        color: "from-green-500 to-blue-500",
    },
];

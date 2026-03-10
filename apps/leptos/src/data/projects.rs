#[derive(Debug, Clone)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub category: &'static str,
    pub tags: &'static [&'static str],
    pub link: &'static str,
    pub color_class: &'static str,
}

pub static FEATURED_PROJECTS: &[Project] = &[
    Project {
        id: "01",
        title: "Rust Infrastructure",
        description: "High-performance backend systems built with memory-safe Rust architecture. Zero-cost abstractions at scale.",
        image: "/public/project-rust.png",
        category: "Backend",
        tags: &["Rust", "Axum", "SeaORM"],
        link: "/project",
        color_class: "text-primary",
    },
    Project {
        id: "02",
        title: "Elysia Discovery",
        description: "Scalable API services featuring ultra-fast response times, full OpenAPI documentation, and sub-millisecond latency.",
        image: "/public/project-elysia.png",
        category: "API",
        tags: &["Bun", "ElysiaJS", "OpenAPI"],
        link: "/project",
        color_class: "text-accent",
    },
    Project {
        id: "03",
        title: "Media Ecosystem",
        description: "Cinematic frontend experiences designed for high-end content delivery, immersive streaming, and seamless interaction.",
        image: "/public/project-anime.png",
        category: "Frontend",
        tags: &["Leptos", "WASM", "SolidJS"],
        link: "/project",
        color_class: "text-purple-400",
    },
];

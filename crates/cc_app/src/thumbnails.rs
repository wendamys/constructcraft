//! Thumbnails: a small render of a project's model, shown on the home screen,
//! and of a component file, shown in the component library; stored in the application
//! data folder.

use crate::state::AppState;
use cc_core::{Project, ViewKind};
use std::path::{Path, PathBuf};

/// Thumbnail size in pixels (3:2).
pub const SIZE: [u32; 2] = [360, 240];

/// A thumbnail waiting to be rendered (rendering needs the GPU, which only the
/// app loop has).
#[derive(Debug, Clone, PartialEq)]
pub enum Job {
    /// The open document, saved at this path.
    Project(PathBuf),
    /// A component file, for the component library (see [`component_file`]).
    Component(PathBuf),
    /// A bundled sample project.
    Sample(&'static str),
}

fn dir() -> Option<PathBuf> {
    crate::recovery::data_dir().map(|d| d.join("thumbnails"))
}

/// FNV-1a: stable file names across runs and platforms.
fn fnv64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3))
}

pub fn project_file(path: &Path) -> Option<PathBuf> {
    let key = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    Some(dir()?.join(format!("{:016x}.png", fnv64(key.to_string_lossy().as_bytes()))))
}

pub fn sample_file(name: &str) -> Option<PathBuf> {
    Some(dir()?.join(format!("sample-{name}.png")))
}

/// Render `project`, whose model geometry is `geom`, framed whole from its first 3D view's direction.
fn render(project: &Project, geom: &cc_geom::ModelGeometry, rs: &egui_wgpu::RenderState) -> Option<Vec<u8>> {
    geom.bounds()?;
    let mut camera = project
        .views
        .values()
        .find_map(|v| match v.kind {
            ViewKind::ThreeD { camera, .. } => Some(camera),
            _ => None,
        })
        .unwrap_or_default();
    crate::view3d::fit_camera(&mut camera, geom, SIZE[0] as f32 / SIZE[1] as f32);
    let mut renderer = cc_render::Renderer3D::new(&rs.device);
    renderer.upload(
        &rs.device,
        &cc_render::SceneMesh::build_plain(&cc_render::Tessellation::new(geom), &Default::default()),
    );
    renderer.render_to_rgba(&rs.device, &rs.queue, SIZE, &camera)
}

fn write_png(path: &Path, rgba: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let mut encoder = png::Encoder::new(file, SIZE[0], SIZE[1]);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()?.write_image_data(rgba)?;
    Ok(())
}

/// The thumbnail of the component file `path`: a 3D render of its first
/// type, shown in the component library's list. Its name follows the
/// file's size and modification time, so a file that changed gets a new one.
pub fn component_file(path: &Path) -> Option<PathBuf> {
    let meta = std::fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos();
    let file = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let key = format!("{}|{}|{modified}", file.to_string_lossy(), meta.len());
    Some(dir()?.join(format!("component-{:016x}.png", fnv64(key.as_bytes()))))
}

/// A new project with one instance of the first type of the component
/// file `path`, to render its thumbnail: placed at the origin, or, a door
/// or a window, in a wall just long enough to take it and a little taller
/// (a window on a low sill). `None` when the
/// file does not load or its component cannot be placed.
pub fn component_scene(path: &Path) -> Option<cc_engine::Session> {
    use serde_json::json;
    // Walls to try for a door or a window, shortest first, in meters.
    const WALLS: [f64; 3] = [1.6, 3.2, 6.4];
    let load = |session: &mut cc_engine::Session| {
        let out = session.execute("component_load", json!({ "path": path })).ok()?;
        let id = out["types"].get(0)?.as_u64()?;
        let category = session.project().element_type(cc_core::ElementTypeId(id))?.category();
        Some((id, category))
    };
    let mut session = cc_engine::Session::default();
    let (id, category) = load(&mut session)?;
    let command = match category {
        cc_core::Category::Door => "door_create",
        cc_core::Category::Window => "window_create",
        _ => {
            session.execute("component_place", json!({ "type": id, "position": [0.0, 0.0] })).ok()?;
            return Some(session);
        }
    };
    for length in WALLS {
        let mut session = cc_engine::Session::default();
        let (id, _) = load(&mut session)?;
        let wall = session.execute("wall_create", json!({ "start": [0.0, 0.0], "end": [length, 0.0] })).ok()?;
        let mut opening = json!({ "wall": wall["id"], "type": id });
        if category == cc_core::Category::Window {
            // A low sill, so that the wall below the window does not dwarf it either.
            opening["sill"] = json!(0.3);
        }
        if session.execute(command, opening).is_ok() {
            // The wall's default 3 m would dwarf the opening: it stops a little above its head.
            let top = session
                .geometry()
                .solids
                .iter()
                .filter(|s| s.category != cc_core::Category::Wall)
                .filter_map(|s| s.bounds())
                .map(|(_, hi)| hi.z)
                .reduce(f64::max);
            if let Some(top) = top.filter(|t| t.is_finite() && *t > 0.0) {
                // Kept as it is when refused.
                let _ = session.execute("element_set", json!({ "id": wall["id"], "params": { "height": top + 0.25 } }));
            }
            return Some(session);
        }
    }
    None
}

/// The part of the picture's limiting dimension a component fills.
const FILL: f64 = 0.85;

/// The renderer's background (`cc_render`'s clear colour), as bytes.
const RENDER_BACKGROUND: [u8; 3] = [214, 224, 237];

/// A camera seeing all of `points` from the front left and a little above,
/// centered and filling [`FILL`] of the picture: fitted to the points
/// themselves, since a bounding sphere leaves tall or flat things small.
fn component_camera(points: &[glam::DVec3]) -> Option<cc_core::CameraState> {
    let (lo, hi) =
        points.iter().fold((glam::DVec3::INFINITY, glam::DVec3::NEG_INFINITY), |(lo, hi), p| (lo.min(*p), hi.max(*p)));
    if !(lo.is_finite() && hi.is_finite()) {
        return None;
    }
    let mut camera = cc_core::CameraState { target: (lo + hi) * 0.5, yaw_deg: 30.0, pitch_deg: 22.0, distance: 1.0 };
    let forward = camera.forward();
    let right = forward.cross(glam::DVec3::Z).normalize_or_zero();
    let up = right.cross(forward);
    let tan_v = crate::view3d::HALF_FOV_DEG.to_radians().tan() * FILL;
    let tan_h = tan_v * SIZE[0] as f64 / SIZE[1] as f64;
    let rel = |p: &glam::DVec3, target: glam::DVec3| {
        let r = *p - target;
        (r.dot(right), r.dot(up), r.dot(forward))
    };
    let fit = |target| {
        let need = |(x, y, z): (f64, f64, f64)| (x.abs() / tan_h).max(y.abs() / tan_v) - z;
        points.iter().map(|p| need(rel(p, target))).fold(0.02, f64::max)
    };
    // The distance fitting every point around the target, then the target moved
    // to the middle of what is seen (a perspective view is not symmetric), twice.
    for _ in 0..2 {
        camera.distance = fit(camera.target);
        let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for (x, y, z) in points.iter().map(|p| rel(p, camera.target)) {
            let depth = (camera.distance + z).max(1e-6);
            (x0, x1, y0, y1) = (x0.min(x / depth), x1.max(x / depth), y0.min(y / depth), y1.max(y / depth));
        }
        camera.target += (right * (x0 + x1) + up * (y0 + y1)) * 0.5 * camera.distance;
    }
    camera.distance = fit(camera.target);
    camera.distance.is_finite().then_some(camera)
}

/// Render the model of `geom` whole on white, seen from the front left and
/// a little above. Unlike [`render`], small things fill the picture too.
fn render_component(geom: &cc_geom::ModelGeometry, device: &wgpu::Device, queue: &wgpu::Queue) -> Option<Vec<u8>> {
    // What the mesh below draws: pieces shown at the medium detail level.
    let shown = geom.solids.iter().filter(|s| s.detail.shows(cc_core::DetailLevel::Medium));
    let points: Vec<_> = shown.flat_map(|s| s.faces.iter().flat_map(|f| f.verts.iter().copied())).collect();
    let camera = component_camera(&points)?;
    let mut renderer = cc_render::Renderer3D::new(device);
    renderer
        .upload(device, &cc_render::SceneMesh::build_plain(&cc_render::Tessellation::new(geom), &Default::default()));
    let mut rgba = renderer.render_to_rgba(device, queue, SIZE, &camera)?;
    // The library's cards are white paper in both themes; the renderer's clear colour is fixed.
    for px in rgba.as_chunks_mut::<4>().0 {
        if px.iter().zip(RENDER_BACKGROUND).all(|(a, b)| a.abs_diff(b) <= 1) {
            px[..3].fill(255);
        }
    }
    Some(rgba)
}

/// The thumbnail of the component file `path`, as RGBA bytes of [`SIZE`].
fn component_rgba(path: &Path, device: &wgpu::Device, queue: &wgpu::Queue) -> Option<Vec<u8>> {
    component_scene(path).and_then(|mut s| render_component(s.geometry(), device, queue))
}

/// Render pending thumbnails: the projects' at once, one component's a
/// frame (its scene takes a while to build), asking `ctx` for another
/// frame while some are left.
pub fn process(state: &mut AppState, rs: &egui_wgpu::RenderState, ctx: &egui::Context) {
    let mut later = Vec::new();
    let mut component_done = false;
    for job in std::mem::take(&mut state.thumbnail_jobs) {
        let (rgba, file) = match &job {
            Job::Component(_) if component_done => {
                later.push(job);
                continue;
            }
            Job::Component(path) => {
                component_done = true;
                (component_rgba(path, &rs.device, &rs.queue), component_file(path))
            }
            Job::Project(path) => {
                // The open document's geometry is the session's, already current or rebuilt once.
                let (project, geom) = state.session.project_and_geometry();
                (render(project, geom, rs), project_file(path))
            }
            Job::Sample(name) => {
                let project = crate::home::sample_project(name);
                let rgba = project.and_then(|p| render(&p, &cc_geom::ModelGeometry::build(&p), rs));
                (rgba, sample_file(name))
            }
        };
        let (Some(rgba), Some(file)) = (rgba, file) else { continue };
        if let Err(err) = write_png(&file, &rgba) {
            log::warn!("could not write thumbnail {}: {err}", file.display());
        }
    }
    // The list shows the thumbnail made, and the next one is made, on the frame after.
    if component_done {
        ctx.request_repaint();
    }
    state.thumbnail_jobs = later;
}

#[cfg(test)]
mod tests {
    use super::*;
    use cc_core::{Category, TypeData};

    /// The component of the sample house's first type of `category`, saved to a file.
    fn saved(category: Category, name: &str) -> Option<PathBuf> {
        let project = crate::home::sample_project("casa_simples").unwrap();
        let mut session = cc_engine::Session::new(cc_core::Document::new(project));
        let component =
            session.project().component_types().filter(|t| t.category() == category).find_map(|t| match &t.data {
                TypeData::Component { component, .. } => Some(component.clone()),
                _ => None,
            })?;
        let dir = std::env::temp_dir().join(format!("ccad-thumbnails-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(name);
        session.run(cc_engine::commands::io::ComponentSave { component, path: path.clone() }).unwrap();
        Some(path)
    }

    /// Thumbnails are kept in the app's own data folder.
    #[test]
    fn thumbnails_are_kept_in_the_apps_data_folder() {
        assert_eq!(dir(), crate::recovery::data_dir().map(|d| d.join("thumbnails")));
    }

    /// A door's thumbnail shows it in a wall; other components stand alone.
    #[test]
    fn a_component_scene_holds_the_component_placed() {
        let door = saved(Category::Door, "door.ccomp").expect("the sample house has a door component");
        let mut scene = component_scene(&door).expect("a scene for the door");
        let count =
            |s: &cc_engine::Session, c: Category| s.project().elements.values().filter(|e| e.category() == c).count();
        assert_eq!((count(&scene, Category::Wall), count(&scene, Category::Door)), (1, 1));
        assert!(scene.geometry().bounds().is_some(), "something to render");
        // A component that is neither a door nor a window is placed alone.
        let project = crate::home::sample_project("casa_simples").unwrap();
        let other = project
            .component_types()
            .map(|t| t.category())
            .find(|c| !matches!(c, Category::Door | Category::Window | Category::TitleBlock));
        if let Some(path) = other.and_then(|c| saved(c, "other.ccomp")) {
            let mut scene = component_scene(&path).expect("a scene for the component");
            assert_eq!((count(&scene, Category::Wall), scene.project().elements.len()), (0, 1));
            assert!(scene.geometry().bounds().is_some());
        }
        // A file that is no component makes no scene.
        let bad = door.with_file_name("bad.ccomp");
        std::fs::write(&bad, "not a component").unwrap();
        assert!(component_scene(&bad).is_none());
    }

    /// A thumbnail's name follows the file's content: a changed file gets another.
    #[test]
    fn a_component_thumbnail_is_named_after_its_file() {
        let dir = std::env::temp_dir().join(format!("ccad-thumbnail-names-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("a.ccomp");
        std::fs::write(&file, "one").unwrap();
        let Some(first) = component_file(&file) else { return };
        assert_eq!(component_file(&file), Some(first.clone()));
        std::fs::write(&file, "longer").unwrap();
        assert_ne!(component_file(&file), Some(first));
        assert!(component_file(&dir.join("missing.ccomp")).is_none());
    }

    /// Renders need a GPU, so they run on request, to look at:
    /// `CC_COMPONENT_THUMBNAILS=<folder of .ccomp files> cargo nextest run -p cc_app
    /// component_thumbnails_render` writes `<name>.png` of each into `CC_THUMBNAILS_OUT`
    /// (default: `target-review/thumbnails` at the workspace root) and prints how much
    /// of the picture the component fills.
    #[test]
    fn component_thumbnails_render() {
        let Some(input) = std::env::var_os("CC_COMPONENT_THUMBNAILS").map(PathBuf::from) else { return };
        let out = std::env::var_os("CC_THUMBNAILS_OUT")
            .map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target-review/thumbnails"));
        fn block_on<F: std::future::Future>(future: F) -> F::Output {
            let mut future = std::pin::pin!(future);
            let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
            loop {
                if let std::task::Poll::Ready(v) = future.as_mut().poll(&mut cx) {
                    return v;
                }
                std::thread::yield_now();
            }
        }
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = block_on(instance.request_adapter(&Default::default())).expect("a GPU adapter");
        let (device, queue) = block_on(adapter.request_device(&Default::default())).expect("a GPU device");
        let mut files: Vec<_> = std::fs::read_dir(&input).unwrap().filter_map(|e| Some(e.ok()?.path())).collect();
        files.retain(|p| p.extension().is_some_and(|e| e == "ccomp"));
        files.sort();
        for file in files {
            let name = file.file_stem().unwrap().to_string_lossy().into_owned();
            let Some(rgba) = component_rgba(&file, &device, &queue) else {
                println!("{name}: no thumbnail");
                continue;
            };
            write_png(&out.join(format!("{name}.png")), &rgba).unwrap();
            // The share of the width and height the pixels unlike the corner's span.
            let corner = &rgba[..4];
            let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0, 0);
            for (i, px) in rgba.as_chunks::<4>().0.iter().enumerate() {
                if px.iter().zip(corner).any(|(a, b)| a.abs_diff(*b) > 8) {
                    let (x, y) = (i as u32 % SIZE[0], i as u32 / SIZE[0]);
                    (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
                }
            }
            let fill = |a: u32, b: u32, size: u32| (b + 1).saturating_sub(a) as f32 / size as f32;
            println!(
                "{name}: width {:.0}%, height {:.0}%, box x {x0}..{x1} y {y0}..{y1}",
                fill(x0, x1, SIZE[0]) * 100.0,
                fill(y0, y1, SIZE[1]) * 100.0
            );
        }
    }
}

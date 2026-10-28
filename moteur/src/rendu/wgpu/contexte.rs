use web_sys::HtmlCanvasElement;

pub struct Contexte {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub adaptateur: wgpu::Adapter,
    pub device: wgpu::Device,
    pub file: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl Contexte {
    #[cfg(target_arch = "wasm32")]
    pub async fn initialiser(canvas: &HtmlCanvasElement) -> Result<Self, String> {
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|e| e.to_string())?;
        let adaptateur = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| "aucun adaptateur graphique compatible".to_string())?;
        let limites = wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adaptateur.limits());
        let desc = wgpu::DeviceDescriptor {
            label: Some("moteur_device"),
            required_features: wgpu::Features::empty(),
            required_limits: limites,
            memory_hints: wgpu::MemoryHints::Performance,
        };
        let (device, file) = adaptateur.request_device(&desc, None).await.map_err(|e| e.to_string())?;
        let capacites = surface.get_capabilities(&adaptateur);
        let format = capacites.formats.iter().copied().find(|f| !f.is_srgb()).unwrap_or(capacites.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: canvas.width().max(1),
            height: canvas.height().max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: capacites.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        Ok(Self { instance, surface, adaptateur, device, file, config })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn initialiser(_canvas: &HtmlCanvasElement) -> Result<Self, String> {
        Err("surface indisponible hors wasm".to_string())
    }

    pub fn redimensionner(&mut self, largeur: u32, hauteur: u32) {
        self.config.width = largeur.max(1);
        self.config.height = hauteur.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn obtenir_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
}

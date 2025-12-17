// TODO: make labels dynamic
// TODO: does bind group layout and bind group need `binding` unique for the whole program?
//

pub struct Texture {
    // TODO: unsure if necessary/a good idea to store image
    pub img: image::DynamicImage,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Texture {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = rgba.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        //
        // Create texture
        //
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            // This is the same as with the SurfaceConfig. It
            // specifies what texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case) is
            // always supported. Note that using a different
            // texture format is not supported on the WebGL2
            // backend.
            view_formats: &[],
        });

        //
        // Copy texture to GPU
        //
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The image data
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        //
        // Create view and sampler for the copied image
        //
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        //
        // Create a bind group layout
        //
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        //
        // Create a bind group
        //
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        Self {
            img,
            texture,
            view,
            sampler,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, img: image::DynamicImage) {
        let width = img.width();
        let height = img.height();
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &image::DynamicImage::ImageRgba8(img.clone().into()).to_rgba8(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        )
    }
}

pub struct TextureArray {
    // TODO: unsure if necessary/a good idea to store images
    pub images: Vec<image::DynamicImage>,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    // pub label: &str,
}

impl TextureArray {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: Vec<image::DynamicImage>,
        // label: &str,
    ) -> Self {
        assert!(
            images.len() > 0,
            "TextureArray must be created with at least 1 image!"
        );

        // TODO: check and handle if images are not the same size
        let width = images[0].width();
        let height = images[0].height();

        let texture = TextureArray::create_texture(
            device,
            /*label,*/ width,
            height,
            images.len() as u32,
        );
        let view = TextureArray::create_view(&texture);
        let sampler = TextureArray::create_sampler(&device);

        for index in 0..images.len() {
            let rgba = images[index].to_rgba8();
            println!("x: {}, y: {}", rgba.width(), rgba.height());

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    // origin z specifies which layer we're writing into
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: index as u32,
                    },
                },
                // The image data
                &rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    // 1 here because this is for a single image into the array
                    depth_or_array_layers: 1,
                },
            );
        }

        let bind_group_layout = TextureArray::create_bind_group_layout(device);

        let bind_group =
            TextureArray::create_bind_group(device, &bind_group_layout, &view, &sampler);

        Self {
            images,
            texture,
            view,
            sampler,
            bind_group_layout,
            bind_group,
        }
    }

    fn create_texture(
        device: &wgpu::Device,
        /*label: Option<&str>,*/
        width: u32,
        height: u32,
        layers: u32,
    ) -> wgpu::Texture {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: layers,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("tmp"),
            /*label,*/
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // Including COPY_SRC means we may copy this to a new texture (e.g. when resizing the texture array)
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        return texture;
    }

    fn create_view(texture: &wgpu::Texture) -> wgpu::TextureView {
        // Use defaults except that it's a D2Array
        return texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
    }

    fn create_sampler(device: &wgpu::Device) -> wgpu::Sampler {
        // Use the same as from the single 2D texture
        return device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
    }

    fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_array_bind_group_layout"),
        });
    }

    fn create_bind_group(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        return device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_array_bind_group"),
        });
    }

    pub fn update_single_index(
        &mut self,
        queue: &wgpu::Queue,
        img: image::DynamicImage,
        index: u32,
    ) {
        // TODO: assert that new image size is not larger than texture size
        //   and pad if needed.

        let width = img.width();
        let height = img.height();

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: index, // which texture index to update
                },
            },
            &image::DynamicImage::ImageRgba8(img.clone().into()).to_rgba8(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1, // how many layers to update
            },
        );
    }

    pub fn add_image(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        new_image: image::DynamicImage,
    ) {
        // TODO: Unsure if it's a good pattern to submit the encoder here or if
        // I should do this work in the render method/with some encoder that's
        // visible outside this method

        let width = self.texture.width();
        let height = self.texture.height();

        // TODO: expand by more than one to amortize copy time
        let new_layers = self.texture.depth_or_array_layers() + 1;

        assert!(
            new_image.width() == width,
            "TextureArray images must all be the same width (and height)"
        );
        assert!(
            new_image.height() == height,
            "TextureArray images must all be the same height (and width)"
        );

        self.images.push(new_image.clone());

        let new_texture =
            TextureArray::create_texture(device, /*self.label,*/ width, height, new_layers);
        let new_view = TextureArray::create_view(&new_texture);
        let new_sampler = TextureArray::create_sampler(&device);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("TextureArray copy encoder"),
        });

        for layer in 0..self.texture.depth_or_array_layers() {
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &new_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }

        queue.submit(std::iter::once(encoder.finish()));

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &new_texture,
                mip_level: 0,
                // origin z specifies which layer we're writing into
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    // after the previous textures because 0-indexed, I think
                    z: self.texture.depth_or_array_layers(),
                },
            },
            // The image data
            &new_image.to_rgba8(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                // 1 here because this is for a single image into the array
                depth_or_array_layers: 1,
            },
        );

        let new_bind_group_layout = TextureArray::create_bind_group_layout(device);
        let new_bind_group = TextureArray::create_bind_group(
            device,
            &new_bind_group_layout,
            &new_view,
            &new_sampler,
        );

        // let old_texture = self.texture;
        // let old_view = self.view;
        // let old_sampler = self.sampler;
        // let old_bind_group_layout = self.bind_group_layout;
        // let old_bind_group = self.bind_group;

        self.texture = new_texture;
        self.view = new_view;
        self.sampler = new_sampler;
        self.bind_group_layout = new_bind_group_layout;
        self.bind_group = new_bind_group;

        // TODO: properly destroy the old ones
    }
}
